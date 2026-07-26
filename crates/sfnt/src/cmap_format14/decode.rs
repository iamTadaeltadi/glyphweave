//! Bounded parsing and resolution of format-14 selector records.

mod tables;

use std::collections::BTreeMap;

use super::CmapFormat14;
use crate::{
    ByteSpan, Cause, Cmap14TableKind, CmapCause, Decoded, Diagnostic, Reader, Stage,
    VariationSelector, VariationSelectorMap,
};
use tables::{DefaultTable, NonDefaultTable};

const FORMAT: u16 = 14;
const HEADER_LENGTH: usize = 10;
const SELECTOR_RECORD_LENGTH: usize = 11;

#[derive(Clone, Copy, Debug)]
pub(super) struct OffsetField {
    pub(super) value: u32,
    pub(super) span: ByteSpan,
}

#[derive(Clone, Copy, Debug)]
struct SelectorRecord {
    selector: VariationSelector,
    raw_selector: u32,
    default: OffsetField,
    non_default: OffsetField,
}

#[derive(Clone, Copy, Debug)]
struct Target {
    kind: Cmap14TableKind,
    span: ByteSpan,
}

pub(super) fn decode(bytes: &[u8], offset: usize) -> Decoded<CmapFormat14> {
    let (span, selector_record_count, records_end, records) = decode_records(bytes, offset)?;
    let variation_selector_map = decode_mappings(bytes, span, records_end, &records)?;
    Ok(CmapFormat14 {
        span,
        selector_record_count,
        variation_selector_map,
    })
}

fn decode_records(
    bytes: &[u8],
    offset: usize,
) -> Decoded<(ByteSpan, u32, usize, Vec<SelectorRecord>)> {
    let available = bytes.len().saturating_sub(offset);
    let mut reader = Reader::at(bytes, offset)?;
    let format = reader.u16()?;
    if format != FORMAT {
        return Err(failure(
            field_span(offset, 0, 2),
            CmapCause::UnsupportedCmapFormat {
                expected: FORMAT,
                found: format,
            },
        ));
    }

    let declared = reader.u32()?;
    let declared_length = usize::try_from(declared).ok();
    if declared_length.is_none_or(|length| length < HEADER_LENGTH || length > available) {
        return Err(failure(
            field_span(offset, 2, 4),
            CmapCause::InvalidCmap14Length {
                declared,
                minimum: HEADER_LENGTH,
                available,
            },
        ));
    }
    let declared_length = declared_length.unwrap_or(HEADER_LENGTH);
    let span =
        ByteSpan::sized(offset, declared_length).unwrap_or_else(|| ByteSpan::empty_at(offset));
    let selector_record_count = reader.u32()?;
    let maximum = (declared_length - HEADER_LENGTH) / SELECTOR_RECORD_LENGTH;
    let count = usize::try_from(selector_record_count).ok();
    if count.is_none_or(|count| count > maximum) {
        return Err(failure(
            field_span(offset, 6, 4),
            CmapCause::Cmap14SelectorCountOutOfBounds {
                count: selector_record_count,
                maximum,
            },
        ));
    }
    let count = count.unwrap_or(0);
    let records_end = HEADER_LENGTH + count * SELECTOR_RECORD_LENGTH;
    let mut records = Vec::with_capacity(count);
    let mut previous = None;
    for index in 0..selector_record_count {
        let selector_span = field_span(reader.offset(), 0, 3);
        let raw_selector = reader.u24()?;
        let selector = match char::from_u32(raw_selector).and_then(VariationSelector::new) {
            Some(selector) if previous.is_none_or(|value| raw_selector > value) => selector,
            _ => {
                return Err(failure(
                    selector_span,
                    CmapCause::InvalidCmap14VariationSelector {
                        index,
                        selector: raw_selector,
                        previous,
                    },
                ));
            }
        };
        let default_span = field_span(reader.offset(), 0, 4);
        let default = OffsetField {
            value: reader.u32()?,
            span: default_span,
        };
        let non_default_span = field_span(reader.offset(), 0, 4);
        let non_default = OffsetField {
            value: reader.u32()?,
            span: non_default_span,
        };
        records.push(SelectorRecord {
            selector,
            raw_selector,
            default,
            non_default,
        });
        previous = Some(raw_selector);
    }
    Ok((span, selector_record_count, records_end, records))
}

fn decode_mappings(
    bytes: &[u8],
    subtable: ByteSpan,
    records_end: usize,
    records: &[SelectorRecord],
) -> Decoded<VariationSelectorMap> {
    let mut default_cache = BTreeMap::<u32, DefaultTable>::new();
    let mut non_default_cache = BTreeMap::<u32, NonDefaultTable>::new();
    let mut targets = BTreeMap::new();
    let mut map = VariationSelectorMap::new();

    for record in records {
        if record.default.value != 0 && !default_cache.contains_key(&record.default.value) {
            let table = tables::decode_default(
                bytes,
                subtable,
                records_end,
                record.raw_selector,
                record.default,
            )?;
            register_target(Cmap14TableKind::Default, table.span, &mut targets)?;
            default_cache.insert(record.default.value, table);
        }
        if record.non_default.value != 0
            && !non_default_cache.contains_key(&record.non_default.value)
        {
            let table = tables::decode_non_default(
                bytes,
                subtable,
                records_end,
                record.raw_selector,
                record.non_default,
            )?;
            register_target(Cmap14TableKind::NonDefault, table.span, &mut targets)?;
            non_default_cache.insert(record.non_default.value, table);
        }

        let defaults = default_cache.get(&record.default.value);
        let non_defaults = non_default_cache.get(&record.non_default.value);
        if let (Some(defaults), Some(non_defaults)) = (defaults, non_defaults) {
            if let Some(base) = conflicting_base(defaults, non_defaults) {
                return Err(failure(
                    non_defaults.span,
                    CmapCause::ConflictingCmap14Mapping {
                        selector: record.raw_selector,
                        codepoint: u32::from(base),
                    },
                ));
            }
        }
        if let Some(defaults) = defaults {
            for range in &defaults.ranges {
                map.insert_default_range(record.selector, range.start, range.end);
            }
        }
        if let Some(non_defaults) = non_defaults {
            for mapping in &non_defaults.mappings {
                map.insert_non_default(record.selector, mapping.base, mapping.glyph);
            }
        }
    }
    Ok(map)
}

fn conflicting_base(defaults: &DefaultTable, non_defaults: &NonDefaultTable) -> Option<char> {
    let mut range_index = 0;
    for mapping in &non_defaults.mappings {
        while defaults
            .ranges
            .get(range_index)
            .is_some_and(|range| range.end < mapping.base)
        {
            range_index += 1;
        }
        if defaults
            .ranges
            .get(range_index)
            .is_some_and(|range| range.start <= mapping.base && mapping.base <= range.end)
        {
            return Some(mapping.base);
        }
    }
    None
}

fn register_target(
    kind: Cmap14TableKind,
    span: ByteSpan,
    targets: &mut BTreeMap<usize, Target>,
) -> Decoded<()> {
    let current = Target { kind, span };
    let preceding = targets
        .range(..=span.start())
        .next_back()
        .map(|(_, target)| *target);
    let following = targets
        .range(span.start()..)
        .next()
        .map(|(_, target)| *target);
    let overlap = preceding
        .filter(|target| target.span.overlaps(span))
        .or_else(|| following.filter(|target| target.span.overlaps(span)));

    if let Some(previous) = overlap {
        let (first, second) = if previous.span.start() <= span.start() {
            (previous, current)
        } else {
            (current, previous)
        };
        return Err(failure(
            second.span,
            CmapCause::OverlappingCmap14Tables {
                first: (first.kind, first.span),
                second: (second.kind, second.span),
            },
        ));
    }
    targets.insert(span.start(), current);
    Ok(())
}

pub(super) const fn failure(span: ByteSpan, cause: CmapCause) -> Diagnostic {
    Diagnostic::at(Stage::Decode, span, Cause::Cmap(cause))
}

pub(super) fn field_span(base: usize, relative: usize, length: usize) -> ByteSpan {
    base.checked_add(relative)
        .and_then(|start| ByteSpan::sized(start, length))
        .unwrap_or_else(|| ByteSpan::empty_at(base))
}

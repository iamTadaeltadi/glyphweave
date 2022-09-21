//! Decoding of offset-referenced default and non-default UVS tables.

use super::{OffsetField, failure, field_span};
use crate::{ByteSpan, Cmap14TableKind, CmapCause, Decoded, GlyphId, Reader};

const COUNT_LENGTH: usize = 4;
const DEFAULT_ENTRY_LENGTH: usize = 4;
const NON_DEFAULT_ENTRY_LENGTH: usize = 5;

#[derive(Clone, Copy, Debug)]
pub(super) struct DefaultRange {
    pub(super) start: char,
    pub(super) end: char,
}

#[derive(Clone, Debug)]
pub(super) struct DefaultTable {
    pub(super) span: ByteSpan,
    pub(super) ranges: Vec<DefaultRange>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct NonDefaultMapping {
    pub(super) base: char,
    pub(super) glyph: GlyphId,
}

#[derive(Clone, Debug)]
pub(super) struct NonDefaultTable {
    pub(super) span: ByteSpan,
    pub(super) mappings: Vec<NonDefaultMapping>,
}

pub(super) fn decode_default(
    bytes: &[u8],
    subtable: ByteSpan,
    records_end: usize,
    selector: u32,
    offset: OffsetField,
) -> Decoded<DefaultTable> {
    let start = target_start(
        subtable,
        records_end,
        selector,
        Cmap14TableKind::Default,
        offset,
    )?;
    let mut reader = Reader::at(bytes, start)?;
    let count_span = field_span(start, 0, COUNT_LENGTH);
    let count = reader.u32()?;
    let (span, capacity) = target_span(
        subtable,
        start,
        count,
        DEFAULT_ENTRY_LENGTH,
        selector,
        Cmap14TableKind::Default,
        count_span,
    )?;
    let mut ranges = Vec::with_capacity(capacity);
    let mut previous_end = None;
    for index in 0..count {
        let range_span = field_span(reader.offset(), 0, DEFAULT_ENTRY_LENGTH);
        let raw_start = reader.u24()?;
        let additional = reader.u8()?;
        let raw_end = raw_start
            .checked_add(u32::from(additional))
            .unwrap_or(u32::MAX);
        let scalars = char::from_u32(raw_start).zip(char::from_u32(raw_end));
        let (start, end) = match scalars {
            Some(scalars)
                if raw_end <= 0x00ff_ffff
                    && previous_end.is_none_or(|preceding| raw_start > preceding) =>
            {
                scalars
            }
            _ => {
                return Err(failure(
                    range_span,
                    CmapCause::InvalidCmap14DefaultRange {
                        selector,
                        index,
                        start: raw_start,
                        end: raw_end,
                        previous_end,
                    },
                ));
            }
        };
        ranges.push(DefaultRange { start, end });
        previous_end = Some(raw_end);
    }
    Ok(DefaultTable { span, ranges })
}

pub(super) fn decode_non_default(
    bytes: &[u8],
    subtable: ByteSpan,
    records_end: usize,
    selector: u32,
    offset: OffsetField,
) -> Decoded<NonDefaultTable> {
    let start = target_start(
        subtable,
        records_end,
        selector,
        Cmap14TableKind::NonDefault,
        offset,
    )?;
    let mut reader = Reader::at(bytes, start)?;
    let count_span = field_span(start, 0, COUNT_LENGTH);
    let count = reader.u32()?;
    let (span, capacity) = target_span(
        subtable,
        start,
        count,
        NON_DEFAULT_ENTRY_LENGTH,
        selector,
        Cmap14TableKind::NonDefault,
        count_span,
    )?;
    let mut mappings = Vec::with_capacity(capacity);
    let mut previous = None;
    for index in 0..count {
        let mapping_span = field_span(reader.offset(), 0, NON_DEFAULT_ENTRY_LENGTH);
        let codepoint = reader.u24()?;
        let glyph = GlyphId::new(reader.u16()?);
        let base = match char::from_u32(codepoint) {
            Some(base) if previous.is_none_or(|preceding| codepoint > preceding) => base,
            _ => {
                return Err(failure(
                    mapping_span,
                    CmapCause::InvalidCmap14NonDefaultMapping {
                        selector,
                        index,
                        codepoint,
                        previous,
                    },
                ));
            }
        };
        mappings.push(NonDefaultMapping { base, glyph });
        previous = Some(codepoint);
    }
    Ok(NonDefaultTable { span, mappings })
}

fn target_start(
    subtable: ByteSpan,
    records_end: usize,
    selector: u32,
    kind: Cmap14TableKind,
    offset: OffsetField,
) -> Decoded<usize> {
    let invalid = || {
        failure(
            offset.span,
            CmapCause::InvalidCmap14Offset {
                selector,
                kind,
                offset: offset.value,
                records_end,
                declared_length: subtable.length(),
            },
        )
    };
    let relative = usize::try_from(offset.value).map_err(|_| invalid())?;
    let start = subtable.start().checked_add(relative).ok_or_else(invalid)?;
    if relative < records_end
        || start
            .checked_add(COUNT_LENGTH)
            .is_none_or(|end| end > subtable.end())
    {
        return Err(invalid());
    }
    Ok(start)
}

fn target_span(
    subtable: ByteSpan,
    start: usize,
    count: u32,
    entry_length: usize,
    selector: u32,
    kind: Cmap14TableKind,
    count_span: ByteSpan,
) -> Decoded<(ByteSpan, usize)> {
    let available = subtable.end().saturating_sub(start + COUNT_LENGTH);
    let maximum = available / entry_length;
    let count_usize = usize::try_from(count).ok();
    if count_usize.is_none_or(|count| count > maximum) {
        return Err(failure(
            count_span,
            CmapCause::Cmap14EntryCountOutOfBounds {
                selector,
                kind,
                count,
                maximum,
            },
        ));
    }
    let count_usize = count_usize.unwrap_or(0);
    let length = COUNT_LENGTH + count_usize * entry_length;
    let span = ByteSpan::sized(start, length).unwrap_or_else(|| ByteSpan::empty_at(start));
    Ok((span, count_usize))
}

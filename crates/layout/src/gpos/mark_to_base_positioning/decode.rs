//! Bounded decoding for `MarkBasePos` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::{BaseRecord, MarkRecord, MarkToBasePositioning};
use crate::{Coverage, gpos::Anchor, layout_error, relative_target_span};

impl MarkToBasePositioning {
    /// Decodes a `MarkBasePos` subtable at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported format, an invalid
    /// Coverage, mark array, base array, or anchor offset, an unsupported anchor format, a mark
    /// class outside `markClassCount`, or a record count that disagrees with a Coverage length.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposMarkToBaseFormat { format },
            ));
        }
        let mark_coverage_position = reader.offset();
        let mark_coverage_offset = reader.u16()?;
        let base_coverage_position = reader.offset();
        let base_coverage_offset = reader.u16()?;
        let mark_class_count = reader.u16()?;
        let mark_array_position = reader.offset();
        let mark_array_offset = reader.u16()?;
        let base_array_position = reader.offset();
        let base_array_offset = reader.u16()?;
        let minimum = reader.offset() - span.start();

        let mark_coverage = decode_coverage(
            bytes,
            span,
            mark_coverage_position,
            mark_coverage_offset,
            minimum,
            LayoutTarget::GposMarkToBaseMarkCoverage,
        )?;
        let base_coverage = decode_coverage(
            bytes,
            span,
            base_coverage_position,
            base_coverage_offset,
            minimum,
            LayoutTarget::GposMarkToBaseBaseCoverage,
        )?;

        let mark_array = relative_target_span(
            span,
            mark_array_position,
            mark_array_offset,
            minimum,
            LayoutTarget::GposMarkToBaseMarkArray,
        )?;
        let marks = decode_mark_array(bytes, mark_array, mark_class_count)?;
        if marks.len() != mark_coverage.len() {
            return Err(layout_error(
                mark_array_position,
                LayoutCause::MismatchedGposMarkToBaseMarkCount {
                    coverage: mark_coverage.len(),
                    records: marks.len(),
                },
            ));
        }

        let base_array = relative_target_span(
            span,
            base_array_position,
            base_array_offset,
            minimum,
            LayoutTarget::GposMarkToBaseBaseArray,
        )?;
        let bases = decode_base_array(bytes, base_array, mark_class_count)?;
        if bases.len() != base_coverage.len() {
            return Err(layout_error(
                base_array_position,
                LayoutCause::MismatchedGposMarkToBaseBaseCount {
                    coverage: base_coverage.len(),
                    records: bases.len(),
                },
            ));
        }

        Ok(Self::new(
            mark_coverage,
            base_coverage,
            mark_class_count,
            marks,
            bases,
        ))
    }
}

/// Resolves and decodes one Coverage table referenced by a `MarkBasePos` header.
fn decode_coverage(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum: usize,
    target: LayoutTarget,
) -> Decoded<Coverage> {
    let coverage = relative_target_span(owner, position, offset, minimum, target)?;
    Coverage::decode(&bytes[..owner.end()], coverage.start())
}

/// Decodes a `MarkArray`: a count-prefixed array of `(markClass, markAnchor)` records.
fn decode_mark_array(
    bytes: &[u8],
    array: ByteSpan,
    mark_class_count: u16,
) -> Decoded<Vec<MarkRecord>> {
    let mut reader = Reader::within(bytes, array)?;
    let count = reader.u16()?;
    let mut entries = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let class_position = reader.offset();
        let class = reader.u16()?;
        let anchor_position = reader.offset();
        let anchor_offset = reader.u16()?;
        entries.push((class_position, class, anchor_position, anchor_offset));
    }
    let minimum = reader.offset() - array.start();

    let mut records = Vec::with_capacity(entries.len());
    for (class_position, class, anchor_position, anchor_offset) in entries {
        if class >= mark_class_count {
            return Err(layout_error(
                class_position,
                LayoutCause::InvalidGposMarkToBaseClass {
                    class,
                    mark_class_count,
                },
            ));
        }
        let anchor = relative_target_span(
            array,
            anchor_position,
            anchor_offset,
            minimum,
            LayoutTarget::GposMarkToBaseMarkAnchor,
        )?;
        records.push(MarkRecord::new(
            class,
            Anchor::decode(&bytes[..array.end()], anchor)?,
        ));
    }
    Ok(records)
}

/// Decodes a `BaseArray`: a count-prefixed array of `markClassCount` optional anchor offsets each.
fn decode_base_array(
    bytes: &[u8],
    array: ByteSpan,
    mark_class_count: u16,
) -> Decoded<Vec<BaseRecord>> {
    let mut reader = Reader::within(bytes, array)?;
    let count = reader.u16()?;
    let columns = usize::from(mark_class_count);
    let mut rows = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let mut row = Vec::with_capacity(columns);
        for _ in 0..columns {
            let position = reader.offset();
            row.push((position, reader.u16()?));
        }
        rows.push(row);
    }
    let minimum = reader.offset() - array.start();

    let mut records = Vec::with_capacity(rows.len());
    for row in rows {
        let mut anchors = Vec::with_capacity(row.len());
        for (position, offset) in row {
            anchors.push(decode_base_anchor(bytes, array, position, offset, minimum)?);
        }
        records.push(BaseRecord::new(anchors));
    }
    Ok(records)
}

/// Resolves one base anchor offset, treating a null offset as an absent anchor.
fn decode_base_anchor(
    bytes: &[u8],
    array: ByteSpan,
    position: usize,
    offset: u16,
    minimum: usize,
) -> Decoded<Option<Anchor>> {
    if offset == 0 {
        return Ok(None);
    }
    let anchor = relative_target_span(
        array,
        position,
        offset,
        minimum,
        LayoutTarget::GposMarkToBaseBaseAnchor,
    )?;
    Anchor::decode(&bytes[..array.end()], anchor).map(Some)
}

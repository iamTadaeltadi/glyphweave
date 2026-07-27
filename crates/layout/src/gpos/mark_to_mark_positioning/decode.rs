//! Bounded decoding for `MarkMarkPos` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::{Mark1Record, Mark2Record, MarkToMarkPositioning};
use crate::{Coverage, gpos::Anchor, layout_error, relative_target_span};

/// The encoded size of the `MarkMarkPos` format 1 header and its first legal relative target.
const HEADER_SIZE: usize = 12;

impl MarkToMarkPositioning {
    /// Decodes a `MarkMarkPos` subtable at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported format, an invalid Coverage,
    /// array, or anchor offset, an unsupported anchor format, a mark class outside the declared
    /// range, or a mark record count that disagrees with its Coverage.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposMarkToMarkFormat { format },
            ));
        }
        let mark1_coverage_position = reader.offset();
        let mark1_coverage_offset = reader.u16()?;
        let mark2_coverage_position = reader.offset();
        let mark2_coverage_offset = reader.u16()?;
        let mark_class_count = reader.u16()?;
        let mark1_array_position = reader.offset();
        let mark1_array_offset = reader.u16()?;
        let mark2_array_position = reader.offset();
        let mark2_array_offset = reader.u16()?;

        let mark1_coverage = decode_coverage(
            bytes,
            span,
            mark1_coverage_position,
            mark1_coverage_offset,
            LayoutTarget::MarkToMarkMark1Coverage,
        )?;
        let mark2_coverage = decode_coverage(
            bytes,
            span,
            mark2_coverage_position,
            mark2_coverage_offset,
            LayoutTarget::MarkToMarkMark2Coverage,
        )?;

        let mark1_array = relative_target_span(
            span,
            mark1_array_position,
            mark1_array_offset,
            HEADER_SIZE,
            LayoutTarget::MarkToMarkMark1Array,
        )?;
        let mark1_records = decode_mark1_array(bytes, mark1_array, mark_class_count)?;
        expect_count(
            mark1_array_position,
            mark1_coverage.len(),
            mark1_records.len(),
            true,
        )?;

        let mark2_array = relative_target_span(
            span,
            mark2_array_position,
            mark2_array_offset,
            HEADER_SIZE,
            LayoutTarget::MarkToMarkMark2Array,
        )?;
        let mark2_records = decode_mark2_array(bytes, mark2_array, mark_class_count)?;
        expect_count(
            mark2_array_position,
            mark2_coverage.len(),
            mark2_records.len(),
            false,
        )?;

        Ok(Self::new(
            mark1_coverage,
            mark2_coverage,
            mark_class_count,
            mark1_records,
            mark2_records,
        ))
    }
}

/// Resolves and decodes one Coverage table referenced by the `MarkMarkPos` header.
fn decode_coverage(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    target: LayoutTarget,
) -> Decoded<Coverage> {
    let coverage = relative_target_span(owner, position, offset, HEADER_SIZE, target)?;
    Coverage::decode(&bytes[..owner.end()], coverage.start())
}

/// Reports a mark record count that disagrees with its parallel Coverage length.
fn expect_count(position: usize, inputs: usize, records: usize, mark1: bool) -> Decoded<()> {
    if inputs == records {
        return Ok(());
    }
    let cause = if mark1 {
        LayoutCause::MismatchedGposMarkToMarkMark1Count { inputs, records }
    } else {
        LayoutCause::MismatchedGposMarkToMarkMark2Count { inputs, records }
    };
    Err(layout_error(position, cause))
}

/// Decodes a `MarkArray`: a length-prefixed array of mark class and mark anchor records.
fn decode_mark1_array(
    bytes: &[u8],
    span: ByteSpan,
    mark_class_count: u16,
) -> Decoded<Vec<Mark1Record>> {
    let mut reader = Reader::within(bytes, span)?;
    let count = reader.u16()?;
    let mut headers = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let class_position = reader.offset();
        let class = reader.u16()?;
        if class >= mark_class_count {
            return Err(layout_error(
                class_position,
                LayoutCause::InvalidGposMarkToMarkClass {
                    class,
                    mark_class_count,
                },
            ));
        }
        let anchor_position = reader.offset();
        let anchor_offset = reader.u16()?;
        headers.push((class, anchor_position, anchor_offset));
    }
    let minimum = reader.offset() - span.start();

    let mut records = Vec::with_capacity(headers.len());
    for (class, anchor_position, anchor_offset) in headers {
        let anchor = relative_target_span(
            span,
            anchor_position,
            anchor_offset,
            minimum,
            LayoutTarget::MarkToMarkMark1Anchor,
        )?;
        let anchor = Anchor::decode(&bytes[..span.end()], anchor)?;
        records.push(Mark1Record::new(class, anchor));
    }
    Ok(records)
}

/// Decodes a `Mark2Array`: a length-prefixed grid of `markClassCount` optional anchors per row.
fn decode_mark2_array(
    bytes: &[u8],
    span: ByteSpan,
    mark_class_count: u16,
) -> Decoded<Vec<Mark2Record>> {
    let columns = usize::from(mark_class_count);
    let mut reader = Reader::within(bytes, span)?;
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let mut row = Vec::with_capacity(columns);
        for _ in 0..columns {
            let position = reader.offset();
            row.push((position, reader.u16()?));
        }
        offsets.push(row);
    }
    let minimum = reader.offset() - span.start();

    let mut records = Vec::with_capacity(offsets.len());
    for row in offsets {
        let mut anchors = Vec::with_capacity(row.len());
        for (position, offset) in row {
            anchors.push(decode_optional_anchor(
                bytes, span, position, offset, minimum,
            )?);
        }
        records.push(Mark2Record::new(anchors));
    }
    Ok(records)
}

/// Resolves one optional mark-2 anchor, treating a null offset as an absent anchor.
fn decode_optional_anchor(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum: usize,
) -> Decoded<Option<Anchor>> {
    if offset == 0 {
        return Ok(None);
    }
    let anchor = relative_target_span(
        owner,
        position,
        offset,
        minimum,
        LayoutTarget::MarkToMarkMark2Anchor,
    )?;
    Anchor::decode(&bytes[..owner.end()], anchor).map(Some)
}

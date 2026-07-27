//! Bounded decoding for `MarkLigPos` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::{LigatureAttachment, MarkLigaturePositioning, MarkLigatureRecord};
use crate::{Coverage, gpos::Anchor, layout_error, relative_target_span};

impl MarkLigaturePositioning {
    /// Decodes a `MarkLigPos` subtable at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported format, an invalid Coverage,
    /// `MarkArray`, `LigatureArray`, or Anchor offset, an out-of-range mark class, or a mark or
    /// ligature record count that disagrees with its Coverage.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposMarkLigatureFormat { format },
            ));
        }
        let mark_coverage_position = reader.offset();
        let mark_coverage_offset = reader.u16()?;
        let ligature_coverage_position = reader.offset();
        let ligature_coverage_offset = reader.u16()?;
        let mark_class_count = reader.u16()?;
        let mark_array_position = reader.offset();
        let mark_array_offset = reader.u16()?;
        let ligature_array_position = reader.offset();
        let ligature_array_offset = reader.u16()?;
        let base = reader.offset() - span.start();

        let mark_coverage = decode_coverage(
            bytes,
            span,
            mark_coverage_position,
            mark_coverage_offset,
            base,
            LayoutTarget::MarkLigatureMarkCoverage,
        )?;
        let ligature_coverage = decode_coverage(
            bytes,
            span,
            ligature_coverage_position,
            ligature_coverage_offset,
            base,
            LayoutTarget::MarkLigatureLigatureCoverage,
        )?;

        let mark_array = relative_target_span(
            span,
            mark_array_position,
            mark_array_offset,
            base,
            LayoutTarget::MarkLigatureMarkArray,
        )?;
        let marks = decode_mark_array(bytes, mark_array, mark_class_count, mark_coverage.len())?;

        let ligature_array = relative_target_span(
            span,
            ligature_array_position,
            ligature_array_offset,
            base,
            LayoutTarget::MarkLigatureLigatureArray,
        )?;
        let ligatures = decode_ligature_array(
            bytes,
            ligature_array,
            mark_class_count,
            ligature_coverage.len(),
        )?;

        Ok(Self::new(
            mark_coverage,
            ligature_coverage,
            mark_class_count,
            marks,
            ligatures,
        ))
    }
}

/// Resolves and decodes a Coverage table referenced by the `MarkLigPos` header.
fn decode_coverage(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    base: usize,
    target: LayoutTarget,
) -> Decoded<Coverage> {
    let coverage = relative_target_span(owner, position, offset, base, target)?;
    Coverage::decode(&bytes[..owner.end()], coverage.start())
}

/// Decodes the `MarkArray`: `markCount` records of a mark class and its required attachment anchor.
fn decode_mark_array(
    bytes: &[u8],
    owner: ByteSpan,
    mark_class_count: u16,
    expected: usize,
) -> Decoded<Vec<MarkLigatureRecord>> {
    let mut reader = Reader::within(bytes, owner)?;
    let count_position = reader.offset();
    let count = reader.u16()?;
    if usize::from(count) != expected {
        return Err(layout_error(
            count_position,
            LayoutCause::MismatchedMarkLigatureMarkCount {
                coverage: expected,
                marks: usize::from(count),
            },
        ));
    }
    let mut records = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let class_position = reader.offset();
        let class = reader.u16()?;
        let anchor_position = reader.offset();
        let anchor_offset = reader.u16()?;
        records.push((class_position, class, anchor_position, anchor_offset));
    }
    let base = reader.offset() - owner.start();

    let mut marks = Vec::with_capacity(records.len());
    for (class_position, class, anchor_position, anchor_offset) in records {
        if class >= mark_class_count {
            return Err(layout_error(
                class_position,
                LayoutCause::MarkLigatureMarkClassRange {
                    class,
                    class_count: mark_class_count,
                },
            ));
        }
        let anchor = relative_target_span(
            owner,
            anchor_position,
            anchor_offset,
            base,
            LayoutTarget::MarkLigatureMarkAnchor,
        )?;
        let anchor = Anchor::decode(&bytes[..owner.end()], anchor)?;
        marks.push(MarkLigatureRecord::new(class, anchor));
    }
    Ok(marks)
}

/// Decodes the `LigatureArray`: `ligatureCount` offsets to per-ligature `LigatureAttach` tables.
fn decode_ligature_array(
    bytes: &[u8],
    owner: ByteSpan,
    mark_class_count: u16,
    expected: usize,
) -> Decoded<Vec<LigatureAttachment>> {
    let mut reader = Reader::within(bytes, owner)?;
    let count_position = reader.offset();
    let count = reader.u16()?;
    if usize::from(count) != expected {
        return Err(layout_error(
            count_position,
            LayoutCause::MismatchedMarkLigatureLigatureCount {
                coverage: expected,
                ligatures: usize::from(count),
            },
        ));
    }
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    let base = reader.offset() - owner.start();

    let mut ligatures = Vec::with_capacity(offsets.len());
    for (position, offset) in offsets {
        let attach = relative_target_span(
            owner,
            position,
            offset,
            base,
            LayoutTarget::MarkLigatureLigatureAttach,
        )?;
        ligatures.push(decode_ligature_attach(bytes, attach, mark_class_count)?);
    }
    Ok(ligatures)
}

/// Decodes one `LigatureAttach`: `componentCount` rows of `markClassCount` optional anchors.
fn decode_ligature_attach(
    bytes: &[u8],
    owner: ByteSpan,
    mark_class_count: u16,
) -> Decoded<LigatureAttachment> {
    let mut reader = Reader::within(bytes, owner)?;
    let component_count = reader.u16()?;
    let class_count = usize::from(mark_class_count);
    let base = usize::from(component_count)
        .checked_mul(class_count)
        .and_then(|cells| cells.checked_mul(2))
        .and_then(|body| body.checked_add(2))
        .unwrap_or(usize::MAX);

    let mut components = Vec::with_capacity(usize::from(component_count));
    for _ in 0..component_count {
        let mut row = Vec::with_capacity(class_count);
        for _ in 0..mark_class_count {
            let position = reader.offset();
            let offset = reader.u16()?;
            row.push(decode_optional_anchor(
                bytes, owner, position, offset, base,
            )?);
        }
        components.push(row);
    }
    Ok(LigatureAttachment::new(components))
}

/// Resolves one component anchor, treating a null offset as an absent attachment point.
fn decode_optional_anchor(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    base: usize,
) -> Decoded<Option<Anchor>> {
    if offset == 0 {
        return Ok(None);
    }
    let anchor = relative_target_span(
        owner,
        position,
        offset,
        base,
        LayoutTarget::MarkLigatureLigatureAnchor,
    )?;
    Anchor::decode(&bytes[..owner.end()], anchor).map(Some)
}

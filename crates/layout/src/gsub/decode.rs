//! Shared bounded targets used by GSUB subtable decoders.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTarget, Reader};

use crate::{Coverage, layout_error, relative_target_span};

pub(super) fn decode_coverage(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum_position: usize,
) -> Decoded<Coverage> {
    let target = relative_target_span(
        owner,
        position,
        offset,
        minimum_position - owner.start(),
        LayoutTarget::SubstitutionCoverage,
    )?;
    Coverage::decode(&bytes[..owner.end()], target.start())
}

pub(super) fn decode_coverages(
    bytes: &[u8],
    owner: ByteSpan,
    offsets: impl IntoIterator<Item = (usize, u16)>,
    minimum_position: usize,
) -> Decoded<Vec<Coverage>> {
    offsets
        .into_iter()
        .map(|(position, offset)| decode_coverage(bytes, owner, position, offset, minimum_position))
        .collect()
}

pub(super) fn decode_sequence_map(
    bytes: &[u8],
    span: ByteSpan,
    lookup_type: u16,
    target_kind: LayoutTarget,
) -> Decoded<Vec<(GlyphId, Vec<GlyphId>)>> {
    let mut reader = Reader::within(bytes, span)?;
    let format_position = reader.offset();
    let format = reader.u16()?;
    if format != 1 {
        return Err(layout_error(
            format_position,
            LayoutCause::UnsupportedGsubFormat {
                lookup_type,
                format,
            },
        ));
    }
    let coverage_position = reader.offset();
    let coverage_offset = reader.u16()?;
    let count_position = reader.offset();
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    let minimum_position = reader.offset();
    let coverage = decode_coverage(
        bytes,
        span,
        coverage_position,
        coverage_offset,
        minimum_position,
    )?;
    if coverage.len() != offsets.len() {
        return Err(layout_error(
            count_position,
            LayoutCause::MismatchedGsubMappingCount {
                lookup_type,
                inputs: coverage.len(),
                records: offsets.len(),
            },
        ));
    }

    let mut mappings = Vec::with_capacity(offsets.len());
    for (input, (position, offset)) in coverage.glyphs().iter().copied().zip(offsets) {
        let target = relative_target_span(
            span,
            position,
            offset,
            minimum_position - span.start(),
            target_kind,
        )?;
        let mut sequence = Reader::within(bytes, target)?;
        let glyph_count = sequence.u16()?;
        let mut outputs = Vec::with_capacity(usize::from(glyph_count));
        for _ in 0..glyph_count {
            outputs.push(GlyphId::new(sequence.u16()?));
        }
        mappings.push((input, outputs));
    }
    Ok(mappings)
}

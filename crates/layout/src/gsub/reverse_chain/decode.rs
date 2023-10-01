//! Bounded decoding for `ReverseChainSingleSubst` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, Reader};

use super::ReverseChainSingleSubstitution;
use crate::{
    SingleSubstitution,
    gsub::decode::{decode_coverage, decode_coverages},
    layout_error,
};

impl ReverseChainSingleSubstitution {
    /// Decodes a `ReverseChainSingleSubst` subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid format, offsets, Coverage, or mapping-count mismatch.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGsubFormat {
                    lookup_type: 8,
                    format,
                },
            ));
        }
        let coverage_position = reader.offset();
        let coverage_offset = reader.u16()?;
        let backtrack_offsets = read_offsets(&mut reader)?;
        let lookahead_offsets = read_offsets(&mut reader)?;
        let count_position = reader.offset();
        let count = reader.u16()?;
        let mut outputs = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            outputs.push(GlyphId::new(reader.u16()?));
        }
        let minimum_position = reader.offset();
        let coverage = decode_coverage(
            bytes,
            span,
            coverage_position,
            coverage_offset,
            minimum_position,
        )?;
        if coverage.len() != outputs.len() {
            return Err(layout_error(
                count_position,
                LayoutCause::MismatchedGsubMappingCount {
                    lookup_type: 8,
                    inputs: coverage.len(),
                    records: outputs.len(),
                },
            ));
        }
        let backtrack = decode_coverages(bytes, span, backtrack_offsets, minimum_position)?;
        let lookahead = decode_coverages(bytes, span, lookahead_offsets, minimum_position)?;
        let substitutions =
            SingleSubstitution::from_mappings(coverage.glyphs().iter().copied().zip(outputs));
        Ok(Self::new(backtrack, substitutions, lookahead))
    }
}

fn read_offsets(reader: &mut Reader<'_>) -> Decoded<Vec<(usize, u16)>> {
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    Ok(offsets)
}

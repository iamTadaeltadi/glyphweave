//! Bounded decoding for MultipleSubst format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTarget, Reader};

use super::MultipleSubstitution;
use crate::{gsub::decode::decode_coverage, layout_error, relative_target_span};

impl MultipleSubstitution {
    /// Decodes a MultipleSubst subtable at the beginning of `span`.
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
                    lookup_type: 2,
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
                    lookup_type: 2,
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
                LayoutTarget::SubstitutionSequence,
            )?;
            let mut sequence = Reader::within(bytes, target)?;
            let glyph_count = sequence.u16()?;
            let mut outputs = Vec::with_capacity(usize::from(glyph_count));
            for _ in 0..glyph_count {
                outputs.push(GlyphId::new(sequence.u16()?));
            }
            mappings.push((input, outputs));
        }
        Ok(Self::from_mappings(mappings))
    }
}

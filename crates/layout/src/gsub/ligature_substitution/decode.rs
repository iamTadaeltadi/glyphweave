//! Bounded dispatch for `LigatureSubst` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::LigatureSubstitution;
use crate::{gsub::decode::decode_coverage, layout_error, relative_target_span};

mod set;

impl LigatureSubstitution {
    /// Decodes a `LigatureSubst` subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid format, offsets, Coverage, counts, or Ligature records.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGsubFormat {
                    lookup_type: 4,
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
                    lookup_type: 4,
                    inputs: coverage.len(),
                    records: offsets.len(),
                },
            ));
        }

        let mut rules = Vec::new();
        for (first, (position, offset)) in coverage.glyphs().iter().copied().zip(offsets) {
            let target = relative_target_span(
                span,
                position,
                offset,
                minimum_position - span.start(),
                LayoutTarget::LigatureSet,
            )?;
            rules.extend(set::decode(bytes, target, first)?);
        }
        Ok(Self::new(rules))
    }
}

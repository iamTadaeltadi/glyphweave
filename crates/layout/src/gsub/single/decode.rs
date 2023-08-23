//! Bounded decoding for both SingleSubst formats.
use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTarget, Reader};
use super::SingleSubstitution;
use crate::{Coverage, layout_error, relative_target_span};

impl SingleSubstitution {
    /// Decodes a SingleSubst subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid offsets, formats, Coverage, or count mismatch.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => Self::decode_delta(bytes, span, &mut reader),
            2 => Self::decode_explicit(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGsubFormat {
                    lookup_type: 1,
                    format,
                },
            )),
        }
    }

    fn decode_delta(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let offset_position = reader.offset();
        let offset = reader.u16()?;
        let delta = reader.i16()?;
        let coverage = decode_coverage(bytes, span, offset_position, offset, reader.offset())?;
        Ok(Self::from_mappings(coverage.glyphs().iter().copied().map(
            |input| {
                let output = i32::from(input.raw()) + i32::from(delta);
                (input, GlyphId::new(output.rem_euclid(65_536) as u16))
            },
        )))
    }

    fn decode_explicit(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let offset_position = reader.offset();
        let offset = reader.u16()?;
        let count = reader.u16()?;
        let mut outputs = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            outputs.push(GlyphId::new(reader.u16()?));
        }
        let coverage = decode_coverage(bytes, span, offset_position, offset, reader.offset())?;
        if coverage.len() != outputs.len() {
            return Err(layout_error(
                offset_position + 2,
                LayoutCause::MismatchedGsubMappingCount {
                    lookup_type: 1,
                    inputs: coverage.len(),
                    records: outputs.len(),
                },
            ));
        }
        Ok(Self::from_mappings(
            coverage.glyphs().iter().copied().zip(outputs),
        ))
    }
}

fn decode_coverage(
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

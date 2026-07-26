//! Bounded decoding for both OpenType Coverage formats.

use glyphweave_sfnt::{ByteSpan, Cause, Decoded, Diagnostic, LayoutCause, Reader, Stage};

use super::Coverage;

impl Coverage {
    /// Decodes a Coverage table beginning at an absolute byte offset.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported selector, or a violated
    /// Coverage ordering invariant.
    pub fn decode(bytes: &[u8], offset: usize) -> Decoded<Self> {
        let mut reader = Reader::at(bytes, offset)?;
        let format_offset = reader.offset();
        match reader.u16()? {
            1 => Self::decode_format_1(&mut reader),
            2 => Self::decode_format_2(&mut reader),
            format => Err(layout_error(
                format_offset,
                LayoutCause::UnsupportedCoverageFormat { format },
            )),
        }
    }

    fn decode_format_1(reader: &mut Reader<'_>) -> Decoded<Self> {
        let count = usize::from(reader.u16()?);
        let mut glyphs = Vec::with_capacity(count);
        for _ in 0..count {
            let glyph_offset = reader.offset();
            let glyph = glyphweave_sfnt::GlyphId::new(reader.u16()?);
            if let Some(previous) = glyphs.last().copied() {
                if glyph <= previous {
                    return Err(layout_error(
                        glyph_offset,
                        LayoutCause::NonIncreasingCoverageGlyph {
                            previous,
                            current: glyph,
                        },
                    ));
                }
            }
            glyphs.push(glyph);
        }
        Ok(Self { glyphs })
    }

    fn decode_format_2(reader: &mut Reader<'_>) -> Decoded<Self> {
        let count = usize::from(reader.u16()?);
        let mut glyphs = Vec::new();
        let mut previous_end = None;
        let mut expected_index = 0_u32;

        for _ in 0..count {
            let range_offset = reader.offset();
            let start = glyphweave_sfnt::GlyphId::new(reader.u16()?);
            let end = glyphweave_sfnt::GlyphId::new(reader.u16()?);
            let index_offset = reader.offset();
            let actual_index = reader.u16()?;

            if start > end {
                return Err(layout_error_sized(
                    range_offset,
                    4,
                    LayoutCause::InvalidCoverageRange { start, end },
                ));
            }
            if let Some(previous_end) = previous_end {
                if start <= previous_end {
                    return Err(layout_error(
                        range_offset,
                        LayoutCause::OverlappingCoverageRange {
                            previous_end,
                            start,
                        },
                    ));
                }
            }
            if u32::from(actual_index) != expected_index {
                return Err(layout_error(
                    index_offset,
                    LayoutCause::DiscontinuousCoverageIndex {
                        expected: expected_index,
                        actual: actual_index,
                    },
                ));
            }

            glyphs.extend((start.raw()..=end.raw()).map(glyphweave_sfnt::GlyphId::new));
            expected_index += u32::from(end.raw()) - u32::from(start.raw()) + 1;
            previous_end = Some(end);
        }
        Ok(Self { glyphs })
    }
}

fn layout_error(offset: usize, cause: LayoutCause) -> Diagnostic {
    layout_error_sized(offset, 2, cause)
}

fn layout_error_sized(offset: usize, size: usize, cause: LayoutCause) -> Diagnostic {
    let span = ByteSpan::sized(offset, size).unwrap_or_else(|| ByteSpan::empty_at(offset));
    Diagnostic::at(Stage::Decode, span, Cause::Layout(cause))
}

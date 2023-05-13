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
}

fn layout_error(offset: usize, cause: LayoutCause) -> Diagnostic {
    let span = ByteSpan::sized(offset, 2).unwrap_or_else(|| ByteSpan::empty_at(offset));
    Diagnostic::at(Stage::Decode, span, Cause::Layout(cause))
}

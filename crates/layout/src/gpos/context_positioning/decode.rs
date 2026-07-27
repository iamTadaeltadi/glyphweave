//! Bounded `ContextPos` format dispatch.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, Reader};

use self::shared::{decode_coverages, decode_records};
use super::ContextPositioning;
use crate::{ContextPositioningRule, layout_error};

mod format_1;
mod format_2;
mod shared;

impl ContextPositioning {
    /// Decodes a `ContextPos` subtable at the beginning of `span`.
    ///
    /// # Errors
    /// For truncation, invalid formats, offsets, Coverage, counts, or action indices.
    ///
    /// # Examples
    /// ```
    /// use glyphweave_layout::ContextPositioning;
    /// use glyphweave_sfnt::{ByteSpan, GlyphId};
    ///
    /// let bytes = [
    ///     0x00, 0x03, 0x00, 0x01, 0x00, 0x01, // format 3, glyphCount 1, posCount 1
    ///     0x00, 0x0c, // Coverage offset
    ///     0x00, 0x00, 0x00, 0x07, // apply Lookup 7 to input position 0
    ///     0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    /// ];
    /// let context =
    ///     ContextPositioning::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();
    ///
    /// let ContextPositioning::CoverageRule(rule) = context else {
    ///     panic!("format 3 retains Coverage predicates");
    /// };
    /// assert_eq!(rule.input()[0].glyphs(), [GlyphId::new(3)]);
    /// ```
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => format_1::decode(bytes, span, &mut reader),
            2 => format_2::decode(bytes, span, &mut reader),
            3 => Self::decode_format_3(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposContextFormat { format },
            )),
        }
    }

    fn decode_format_3(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let count_position = reader.offset();
        let glyph_count = reader.u16()?;
        if glyph_count == 0 {
            return Err(layout_error(
                count_position,
                LayoutCause::EmptyGposContextInput,
            ));
        }
        let record_count = reader.u16()?;
        let mut offsets = Vec::with_capacity(usize::from(glyph_count));
        for _ in 0..glyph_count {
            let position = reader.offset();
            offsets.push((position, reader.u16()?));
        }
        let records = decode_records(reader, record_count, glyph_count)?;
        let mut coverages = decode_coverages(bytes, span, offsets, reader.offset())?.into_iter();
        let first = coverages
            .next()
            .expect("nonzero glyph count decoded one Coverage");
        Ok(Self::CoverageRule(ContextPositioningRule::new(
            first, coverages, records,
        )))
    }
}

//! Bounded ContextSubst format dispatch.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, Reader};

use super::ContextSubstitution;
use crate::{
    ContextRule,
    gsub::decode::{decode_coverages, decode_records},
    layout_error,
};

impl ContextSubstitution {
    /// Decodes a ContextSubst subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid formats, offsets, Coverage, counts, or action indices.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            3 => Self::decode_format_3(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGsubFormat {
                    lookup_type: 5,
                    format,
                },
            )),
        }
    }

    fn decode_format_3(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let count_position = reader.offset();
        let glyph_count = reader.u16()?;
        if glyph_count == 0 {
            return Err(layout_error(
                count_position,
                LayoutCause::EmptyContextInput { lookup_type: 5 },
            ));
        }
        let record_count = reader.u16()?;
        let mut offsets = Vec::with_capacity(usize::from(glyph_count));
        for _ in 0..glyph_count {
            let position = reader.offset();
            offsets.push((position, reader.u16()?));
        }
        let records = decode_records(reader, record_count, glyph_count, 5)?;
        let mut coverages = decode_coverages(bytes, span, offsets, reader.offset())?.into_iter();
        let first = coverages
            .next()
            .expect("nonzero glyph count decoded one Coverage");
        Ok(Self::CoverageRule(ContextRule::new(
            first, coverages, records,
        )))
    }
}

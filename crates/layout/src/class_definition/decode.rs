//! Bounded decoding for OpenType Class Definition formats.

use std::collections::BTreeMap;

use glyphweave_sfnt::{Decoded, GlyphId, LayoutCause, Reader};

use super::ClassDefinition;
use crate::{layout_error, layout_error_sized};

impl ClassDefinition {
    /// Decodes a Class Definition table beginning at an absolute byte offset.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported selector, range overflow,
    /// reversed bounds, or overlapping ranges.
    pub fn decode(bytes: &[u8], offset: usize) -> Decoded<Self> {
        let mut reader = Reader::at(bytes, offset)?;
        let format_offset = reader.offset();
        match reader.u16()? {
            1 => Self::decode_format_1(&mut reader),
            format => Err(layout_error(
                format_offset,
                LayoutCause::UnsupportedClassDefinitionFormat { format },
            )),
        }
    }

    fn decode_format_1(reader: &mut Reader<'_>) -> Decoded<Self> {
        let range_offset = reader.offset();
        let start = GlyphId::new(reader.u16()?);
        let count = reader.u16()?;
        if count > 0 && u32::from(start.raw()) + u32::from(count) > 65_536 {
            return Err(layout_error_sized(
                range_offset,
                4,
                LayoutCause::ClassDefinitionGlyphRangeOverflow { start, count },
            ));
        }

        let mut classes = BTreeMap::new();
        for index in 0..count {
            let class = reader.u16()?;
            if class != 0 {
                let raw = start.raw() + index;
                classes.insert(GlyphId::new(raw), class);
            }
        }
        Ok(Self { classes })
    }
}

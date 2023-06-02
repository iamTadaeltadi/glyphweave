//! Deterministic shortest-format Class Definition encoding.

use glyphweave_sfnt::GlyphId;

use super::ClassDefinition;
use crate::push_u16;

#[derive(Clone, Copy)]
struct ClassRange {
    start: GlyphId,
    end: GlyphId,
    class: u16,
}

impl ClassDefinition {
    /// Encodes this canonical Class Definition using the shortest legal OpenType format.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let ranges = self.maximal_ranges();
        let format_2_size = 4 + 6 * ranges.len();
        if let (Some((&start, _)), Some((&end, _))) = (
            self.classes.first_key_value(),
            self.classes.last_key_value(),
        ) {
            let span = usize::from(end.raw()) - usize::from(start.raw()) + 1;
            if let Ok(count) = u16::try_from(span) {
                if 6 + 2 * span <= format_2_size {
                    return self.encode_format_1(start, count);
                }
            }
        }
        Self::encode_format_2(&ranges)
    }

    fn maximal_ranges(&self) -> Vec<ClassRange> {
        let mut ranges: Vec<ClassRange> = Vec::new();
        for (&glyph, &class) in &self.classes {
            match ranges.last_mut() {
                Some(range)
                    if range.class == class
                        && range.end.raw().checked_add(1) == Some(glyph.raw()) =>
                {
                    range.end = glyph;
                }
                _ => ranges.push(ClassRange {
                    start: glyph,
                    end: glyph,
                    class,
                }),
            }
        }
        ranges
    }

    fn encode_format_1(&self, start: GlyphId, count: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(6 + 2 * usize::from(count));
        push_u16(&mut bytes, 1);
        push_u16(&mut bytes, start.raw());
        push_u16(&mut bytes, count);
        for offset in 0..count {
            push_u16(
                &mut bytes,
                self.class_of(GlyphId::new(start.raw() + offset)),
            );
        }
        bytes
    }

    fn encode_format_2(ranges: &[ClassRange]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + 6 * ranges.len());
        push_u16(&mut bytes, 2);
        push_u16(
            &mut bytes,
            u16::try_from(ranges.len()).expect("disjoint u16 ranges fit a u16 count"),
        );
        for range in ranges {
            push_u16(&mut bytes, range.start.raw());
            push_u16(&mut bytes, range.end.raw());
            push_u16(&mut bytes, range.class);
        }
        bytes
    }
}

//! Deterministic shortest-format Coverage encoding.

use glyphweave_sfnt::GlyphId;

use super::Coverage;

#[derive(Clone, Copy)]
struct Range {
    start: GlyphId,
    end: GlyphId,
    start_index: u16,
}

impl Coverage {
    /// Encodes this canonical Coverage using the shortest legal OpenType format.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let ranges = self.maximal_ranges();
        let format_2_size = 4 + 6 * ranges.len();
        if let Ok(count) = u16::try_from(self.len()) {
            let format_1_size = 4 + 2 * self.len();
            if format_1_size <= format_2_size {
                return self.encode_format_1(count);
            }
        }
        Self::encode_format_2(&ranges)
    }

    fn maximal_ranges(&self) -> Vec<Range> {
        let mut ranges: Vec<Range> = Vec::new();
        for (index, glyph) in self.glyphs.iter().copied().enumerate() {
            match ranges.last_mut() {
                Some(range) if range.end.raw().checked_add(1) == Some(glyph.raw()) => {
                    range.end = glyph;
                }
                _ => ranges.push(Range {
                    start: glyph,
                    end: glyph,
                    start_index: u16::try_from(index)
                        .expect("Coverage has at most 65,536 unique glyphs"),
                }),
            }
        }
        ranges
    }

    fn encode_format_1(&self, count: u16) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + 2 * self.len());
        push_u16(&mut bytes, 1);
        push_u16(&mut bytes, count);
        for glyph in &self.glyphs {
            push_u16(&mut bytes, glyph.raw());
        }
        bytes
    }

    fn encode_format_2(ranges: &[Range]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + 6 * ranges.len());
        push_u16(&mut bytes, 2);
        push_u16(
            &mut bytes,
            u16::try_from(ranges.len()).expect("disjoint u16 ranges fit a u16 count"),
        );
        for range in ranges {
            push_u16(&mut bytes, range.start.raw());
            push_u16(&mut bytes, range.end.raw());
            push_u16(&mut bytes, range.start_index);
        }
        bytes
    }
}

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

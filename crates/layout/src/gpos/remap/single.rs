//! Remapping single-glyph GPOS adjustments (lookup type 1).

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::SinglePositioning;

pub(super) fn positioning(
    input: &SinglePositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> SinglePositioning<NewGlyphId> {
    SinglePositioning::new(
        input.value_format(),
        input
            .adjustments()
            .filter_map(|(glyph, record)| mapping(glyph).map(|new| (new, record))),
    )
}

//! Remapping cursive attachment positioning (lookup type 3).

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::CursivePositioning;

pub(super) fn positioning(
    input: &CursivePositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> CursivePositioning<NewGlyphId> {
    let (coverage, records) = super::parallel(input.coverage(), input.records(), mapping);
    CursivePositioning::new(coverage, records)
}

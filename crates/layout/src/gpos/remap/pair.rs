//! Remapping pair positioning (lookup type 2).

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::{Coverage, PairPositioning, PairValue};

pub(super) fn positioning(
    input: &PairPositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> PairPositioning<NewGlyphId> {
    match input {
        PairPositioning::Glyphs { coverage, sets } => glyphs(coverage, sets, mapping),
        PairPositioning::Classes {
            coverage,
            first_classes,
            second_classes,
            grid,
        } => PairPositioning::Classes {
            coverage: super::coverage(coverage, mapping),
            first_classes: super::classes(first_classes, mapping),
            second_classes: super::classes(second_classes, mapping),
            grid: grid.clone(),
        },
    }
}

/// Rewrites format 1, dropping an absent leading glyph's whole set and each absent trailing glyph.
fn glyphs(
    coverage: &Coverage,
    sets: &[Vec<PairValue>],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> PairPositioning<NewGlyphId> {
    let mut paired: Vec<(NewGlyphId, Vec<PairValue<NewGlyphId>>)> = coverage
        .glyphs()
        .iter()
        .zip(sets)
        .filter_map(|(&leading, set)| {
            let new = mapping(leading)?;
            Some((new, remap_set(set, mapping)))
        })
        .collect();
    paired.sort_by_key(|entry| entry.0);
    let coverage = paired.iter().map(|entry| entry.0).collect();
    let sets = paired.into_iter().map(|entry| entry.1).collect();
    PairPositioning::Glyphs { coverage, sets }
}

/// Rewrites one `PairSet`, dropping every record whose trailing glyph the subset omits.
fn remap_set(
    set: &[PairValue],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Vec<PairValue<NewGlyphId>> {
    set.iter()
        .filter_map(|value| {
            mapping(value.second()).map(|second| PairValue::new(second, value.adjustment()))
        })
        .collect()
}

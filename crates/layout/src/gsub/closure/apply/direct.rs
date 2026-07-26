//! Monotone output growth for direct substitution subtables.

use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use crate::{Coverage, GsubSubtable};

#[cfg(test)]
mod tests;

pub(super) fn apply(subtable: &GsubSubtable, glyphs: &mut BTreeSet<GlyphId>) {
    let outputs = match subtable {
        GsubSubtable::Single(value) => value
            .mappings()
            .filter(|(input, _)| glyphs.contains(input))
            .map(|(_, output)| output)
            .collect(),
        GsubSubtable::Multiple(value) => value
            .mappings()
            .filter(|(input, _)| glyphs.contains(input))
            .flat_map(|(_, outputs)| outputs.iter().copied())
            .collect(),
        GsubSubtable::Alternate(value) => value
            .mappings()
            .filter(|(input, _)| glyphs.contains(input))
            .flat_map(|(_, outputs)| outputs.iter().copied())
            .collect(),
        GsubSubtable::Ligature(value) => value
            .rules()
            .iter()
            .filter(|rule| rule.components().iter().all(|glyph| glyphs.contains(glyph)))
            .map(crate::LigatureRule::replacement)
            .collect(),
        GsubSubtable::ReverseChainSingle(value)
            if coverages_match(value.backtrack(), glyphs)
                && coverages_match(value.lookahead(), glyphs) =>
        {
            value
                .substitutions()
                .mappings()
                .filter(|(input, _)| glyphs.contains(input))
                .map(|(_, output)| output)
                .collect()
        }
        _ => Vec::new(),
    };
    glyphs.extend(outputs);
}

pub(super) fn coverage_matches(coverage: &Coverage, glyphs: &BTreeSet<GlyphId>) -> bool {
    coverage.glyphs().iter().any(|glyph| glyphs.contains(glyph))
}

pub(super) fn coverages_match(coverages: &[Coverage], glyphs: &BTreeSet<GlyphId>) -> bool {
    coverages
        .iter()
        .all(|coverage| coverage_matches(coverage, glyphs))
}

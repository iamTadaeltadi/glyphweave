//! Remapping for reverse-chaining single substitutions.

use glyphweave_sfnt::{Decoded, GlyphId, NewGlyphId};

use crate::{Coverage, ReverseChainSingleSubstitution};

pub(super) fn substitution(
    input: &ReverseChainSingleSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<ReverseChainSingleSubstitution<NewGlyphId>> {
    let backtrack = coverages(input.backtrack(), mapping)?;
    let substitutions = super::direct::single(input.substitutions(), mapping)?;
    let lookahead = coverages(input.lookahead(), mapping)?;
    Ok(ReverseChainSingleSubstitution::new(
        backtrack,
        substitutions,
        lookahead,
    ))
}

fn coverages(
    input: &[Coverage],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<Vec<Coverage<NewGlyphId>>> {
    input
        .iter()
        .map(|coverage| super::coverage(coverage, mapping))
        .collect()
}

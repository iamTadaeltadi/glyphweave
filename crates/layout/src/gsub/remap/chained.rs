//! Remapping for chained-contextual predicate domains.

use glyphweave_sfnt::{Decoded, GlyphId, NewGlyphId};

use crate::ChainedContextSubstitution;

pub(super) fn substitution(
    input: &ChainedContextSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<ChainedContextSubstitution<NewGlyphId>> {
    match input {
        ChainedContextSubstitution::GlyphRules(rules) => rules
            .iter()
            .map(|rule| super::rule(rule, 6, |input| super::glyph(*input, mapping)))
            .collect::<Decoded<Vec<_>>>()
            .map(ChainedContextSubstitution::GlyphRules),
        ChainedContextSubstitution::ClassRules {
            coverage,
            backtrack_classes,
            input_classes,
            lookahead_classes,
            rules,
        } => Ok(ChainedContextSubstitution::ClassRules {
            coverage: super::coverage(coverage, mapping)?,
            backtrack_classes: super::classes(backtrack_classes, mapping)?,
            input_classes: super::classes(input_classes, mapping)?,
            lookahead_classes: super::classes(lookahead_classes, mapping)?,
            rules: rules.clone(),
        }),
        ChainedContextSubstitution::CoverageRule(rule) => {
            super::rule(rule, 6, |input| super::coverage(input, mapping))
                .map(ChainedContextSubstitution::CoverageRule)
        }
    }
}

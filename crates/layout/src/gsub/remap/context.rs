//! Remapping for contextual substitution predicate domains.

use glyphweave_sfnt::{Decoded, GlyphId, NewGlyphId};

use crate::ContextSubstitution;

pub(super) fn substitution(
    input: &ContextSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<ContextSubstitution<NewGlyphId>> {
    match input {
        ContextSubstitution::GlyphRules(rules) => rules
            .iter()
            .map(|rule| super::rule(rule, 5, |input| super::glyph(*input, mapping)))
            .collect::<Decoded<Vec<_>>>()
            .map(ContextSubstitution::GlyphRules),
        ContextSubstitution::ClassRules {
            coverage,
            classes,
            rules,
        } => Ok(ContextSubstitution::ClassRules {
            coverage: super::coverage(coverage, mapping)?,
            classes: super::classes(classes, mapping)?,
            rules: rules.clone(),
        }),
        ContextSubstitution::CoverageRule(rule) => {
            super::rule(rule, 5, |input| super::coverage(input, mapping))
                .map(ContextSubstitution::CoverageRule)
        }
    }
}

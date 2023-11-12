//! Semantic forms of chained contextual substitution.

use glyphweave_sfnt::GlyphId;

use crate::{ClassDefinition, ContextRule, Coverage};

mod decode;

#[cfg(test)]
mod tests;

/// A `ChainContextSubst` subtable in its exact-glyph, class, or Coverage predicate domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainedContextSubstitution<G = GlyphId> {
    /// Format 1 rules expanded with their Coverage-selected first input glyph.
    GlyphRules(Vec<ContextRule<G>>),
    /// Format 2 rules retaining first-glyph Coverage and three Class Definitions.
    ClassRules {
        /// Glyphs eligible for the first input position.
        coverage: Coverage<G>,
        /// Class assignments for backtrack predicates.
        backtrack_classes: ClassDefinition<G>,
        /// Class assignments for input predicates.
        input_classes: ClassDefinition<G>,
        /// Class assignments for lookahead predicates.
        lookahead_classes: ClassDefinition<G>,
        /// Class rules in `ChainSubClassSet` and encoded preference order.
        rules: Vec<ContextRule<u16>>,
    },
    /// One format 3 rule with independent Coverage predicates at every position.
    CoverageRule(ContextRule<Coverage<G>>),
}

impl<G> ChainedContextSubstitution<G> {
    /// Returns whether the subtable contains no rule that can invoke a nested Lookup.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::GlyphRules(rules) => rules.is_empty(),
            Self::ClassRules { rules, .. } => rules.is_empty(),
            Self::CoverageRule(rule) => rule.records().is_empty(),
        }
    }

    /// Returns the number of semantic rules represented by this subtable.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::GlyphRules(rules) => rules.len(),
            Self::ClassRules { rules, .. } => rules.len(),
            Self::CoverageRule(_) => 1,
        }
    }
}

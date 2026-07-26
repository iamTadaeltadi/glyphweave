//! Semantic forms of contextual substitution.

use glyphweave_sfnt::GlyphId;

use crate::{ClassDefinition, ContextRule, Coverage};

mod decode;

/// A `ContextSubst` subtable represented in its exact-glyph, class, or Coverage predicate domain.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextSubstitution {
    /// Format 1 rules expanded with their Coverage-selected first glyph.
    GlyphRules(Vec<ContextRule<GlyphId>>),
    /// Format 2 rules retaining first-glyph Coverage and one shared input Class Definition.
    ClassRules {
        /// Glyphs eligible for the first input position.
        coverage: Coverage,
        /// Class assignments used by every input predicate.
        classes: ClassDefinition,
        /// Class rules in `ClassSet` and encoded preference order.
        rules: Vec<ContextRule<u16>>,
    },
    /// One format 3 rule with an independent Coverage for each input position.
    CoverageRule(ContextRule<Coverage>),
}

impl ContextSubstitution {
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

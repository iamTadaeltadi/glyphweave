//! Semantic forms of contextual positioning.

use glyphweave_sfnt::GlyphId;

use crate::{ClassDefinition, Coverage};

mod context_rule;
mod decode;
mod positioning_record;

#[cfg(test)]
mod tests;

pub use context_rule::ContextPositioningRule;
pub use positioning_record::ContextPositioningRecord;

/// A `ContextPos` subtable represented in its exact-glyph, class, or Coverage predicate domain.
///
/// Lookup type 7 contextual positioning shares its byte layout with `ContextSubst`:
/// each rule stores glyph, class, or Coverage input predicates plus GPOS
/// `SequenceLookupRecord`s whose `LookupList` indices are retained as data.
///
/// # Examples
/// ```
/// use glyphweave_layout::ContextPositioning;
/// use glyphweave_sfnt::GlyphId;
///
/// let empty = ContextPositioning::<GlyphId>::GlyphRules(Vec::new());
/// assert!(empty.is_empty());
/// assert_eq!(empty.len(), 0);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextPositioning<G = GlyphId> {
    /// Format 1 rules expanded with their Coverage-selected first glyph.
    GlyphRules(Vec<ContextPositioningRule<G>>),
    /// Format 2 rules retaining first-glyph Coverage and one shared input Class Definition.
    ClassRules {
        /// Glyphs eligible for the first input position.
        coverage: Coverage<G>,
        /// Class assignments used by every input predicate.
        classes: ClassDefinition<G>,
        /// Class rules in `PosClassSet` and encoded preference order.
        rules: Vec<ContextPositioningRule<u16>>,
    },
    /// One format 3 rule with an independent Coverage for each input position.
    CoverageRule(ContextPositioningRule<Coverage<G>>),
}

impl<G> ContextPositioning<G> {
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

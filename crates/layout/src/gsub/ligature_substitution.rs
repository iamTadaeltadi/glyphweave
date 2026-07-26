//! Ordered GSUB ligature rule sets.

use glyphweave_sfnt::GlyphId;

use super::LigatureRule;

mod decode;

/// Ligature rules in their original Coverage-set and preference order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LigatureSubstitution {
    rules: Vec<LigatureRule>,
}

impl LigatureSubstitution {
    /// Builds a substitution without reordering or deduplicating preference-sensitive rules.
    #[must_use]
    pub fn new(rules: impl IntoIterator<Item = LigatureRule>) -> Self {
        Self {
            rules: rules.into_iter().collect(),
        }
    }

    /// Returns every rule in shaping preference order.
    #[must_use]
    pub fn rules(&self) -> &[LigatureRule] {
        &self.rules
    }

    /// Iterates only rules selected by one first component without changing their order.
    pub fn rules_for(&self, first: GlyphId) -> impl Iterator<Item = &LigatureRule> {
        self.rules.iter().filter(move |rule| rule.first() == first)
    }

    /// Returns the number of ligature rules.
    #[must_use]
    pub fn len(&self) -> usize {
        self.rules.len()
    }

    /// Returns whether no ligature rule remains.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::{LigatureRule, LigatureSubstitution};

    #[test]
    fn filters_first_components_without_changing_preference_order() {
        let first = LigatureRule::new(GlyphId::new(1), GlyphId::new(2), [], GlyphId::new(10));
        let unrelated = LigatureRule::new(GlyphId::new(3), GlyphId::new(4), [], GlyphId::new(11));
        let second = LigatureRule::new(GlyphId::new(1), GlyphId::new(5), [], GlyphId::new(12));
        let substitutions = LigatureSubstitution::new([first.clone(), unrelated, second.clone()]);
        assert_eq!(
            substitutions.rules_for(GlyphId::new(1)).collect::<Vec<_>>(),
            [&first, &second,]
        );
        assert_eq!(substitutions.len(), 3);
        assert!(!substitutions.is_empty());
        assert!(LigatureSubstitution::default().is_empty());
    }
}

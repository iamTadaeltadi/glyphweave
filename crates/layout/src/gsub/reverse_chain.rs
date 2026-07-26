//! Reverse-chaining one-to-one substitutions.

use crate::{Coverage, SingleSubstitution};

mod decode;

/// One input map guarded by ordered backtrack and lookahead Coverages.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReverseChainSingleSubstitution {
    backtrack: Vec<Coverage>,
    substitutions: SingleSubstitution,
    lookahead: Vec<Coverage>,
}

impl ReverseChainSingleSubstitution {
    /// Builds a rule without changing context sequence order.
    #[must_use]
    pub fn new(
        backtrack: impl IntoIterator<Item = Coverage>,
        substitutions: SingleSubstitution,
        lookahead: impl IntoIterator<Item = Coverage>,
    ) -> Self {
        Self {
            backtrack: backtrack.into_iter().collect(),
            substitutions,
            lookahead: lookahead.into_iter().collect(),
        }
    }

    /// Returns backtrack Coverages in encoded nearest-to-farthest order.
    #[must_use]
    pub fn backtrack(&self) -> &[Coverage] {
        &self.backtrack
    }

    /// Returns the input-to-output substitution map.
    #[must_use]
    pub const fn substitutions(&self) -> &SingleSubstitution {
        &self.substitutions
    }

    /// Returns lookahead Coverages in input sequence order.
    #[must_use]
    pub fn lookahead(&self) -> &[Coverage] {
        &self.lookahead
    }

    /// Returns whether the input substitution map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.substitutions.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::ReverseChainSingleSubstitution;
    use crate::{Coverage, SingleSubstitution};

    #[test]
    fn preserves_context_direction_and_substitution_map() {
        let backtrack = Coverage::from_glyphs([GlyphId::new(2)]);
        let lookahead = Coverage::from_glyphs([GlyphId::new(8)]);
        let mappings = SingleSubstitution::from_mappings([(GlyphId::new(4), GlyphId::new(14))]);
        let substitution =
            ReverseChainSingleSubstitution::new([backtrack.clone()], mappings, [lookahead.clone()]);
        assert_eq!(substitution.backtrack(), [backtrack]);
        assert_eq!(substitution.lookahead(), [lookahead]);
        assert_eq!(
            substitution.substitutions().replacement(GlyphId::new(4)),
            Some(GlyphId::new(14))
        );
        assert!(!substitution.is_empty());
        assert!(ReverseChainSingleSubstitution::default().is_empty());
    }
}

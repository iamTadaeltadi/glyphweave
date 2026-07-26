//! Canonical one-to-one glyph substitutions.

use std::collections::BTreeMap;

use glyphweave_sfnt::GlyphId;

/// An input-ordered map replacing one glyph with one glyph.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SingleSubstitution {
    mappings: BTreeMap<GlyphId, GlyphId>,
}

impl SingleSubstitution {
    /// Builds canonical mappings; a later duplicate input replaces an earlier mapping.
    #[must_use]
    pub fn from_mappings(mappings: impl IntoIterator<Item = (GlyphId, GlyphId)>) -> Self {
        Self {
            mappings: mappings.into_iter().collect(),
        }
    }

    /// Returns the replacement for one input glyph.
    #[must_use]
    pub fn replacement(&self, input: GlyphId) -> Option<GlyphId> {
        self.mappings.get(&input).copied()
    }

    /// Iterates mappings in ascending input Glyph ID order.
    pub fn mappings(&self) -> impl ExactSizeIterator<Item = (GlyphId, GlyphId)> + '_ {
        self.mappings
            .iter()
            .map(|(&input, &output)| (input, output))
    }

    /// Returns the number of distinct input glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether no input glyph has a replacement.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::SingleSubstitution;

    #[test]
    fn canonicalizes_input_order_and_duplicate_mappings() {
        let substitutions = SingleSubstitution::from_mappings([
            (GlyphId::new(9), GlyphId::new(20)),
            (GlyphId::new(3), GlyphId::new(8)),
            (GlyphId::new(9), GlyphId::new(21)),
        ]);
        assert_eq!(
            substitutions.replacement(GlyphId::new(9)),
            Some(GlyphId::new(21))
        );
        assert_eq!(substitutions.replacement(GlyphId::new(4)), None);
        assert_eq!(
            substitutions.mappings().collect::<Vec<_>>(),
            [
                (GlyphId::new(3), GlyphId::new(8)),
                (GlyphId::new(9), GlyphId::new(21)),
            ]
        );
        assert_eq!(substitutions.len(), 2);
        assert!(!substitutions.is_empty());
        assert!(SingleSubstitution::default().is_empty());
    }
}

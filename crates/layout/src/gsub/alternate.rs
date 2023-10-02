//! Canonical alternate glyph substitutions.

use glyphweave_sfnt::GlyphId;

use super::sequence_map::SequenceMap;

mod decode;

/// An input-ordered map from one glyph to its ordered alternate choices.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AlternateSubstitution {
    mappings: SequenceMap,
}

impl AlternateSubstitution {
    /// Builds canonical alternate sets; a later duplicate input replaces an earlier set.
    #[must_use]
    pub fn from_alternates(alternates: impl IntoIterator<Item = (GlyphId, Vec<GlyphId>)>) -> Self {
        Self {
            mappings: SequenceMap::new(alternates),
        }
    }

    /// Returns one input glyph's ordered alternate choices.
    #[must_use]
    pub fn alternates(&self, input: GlyphId) -> Option<&[GlyphId]> {
        self.mappings.get(input)
    }

    /// Iterates alternate sets in ascending input Glyph ID order.
    #[must_use]
    pub fn mappings(&self) -> impl ExactSizeIterator<Item = (GlyphId, &[GlyphId])> + '_ {
        self.mappings.iter()
    }

    /// Returns the number of distinct input glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether no input glyph has alternates.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::AlternateSubstitution;

    #[test]
    fn preserves_alternate_preference_order() {
        let substitutions = AlternateSubstitution::from_alternates([
            (GlyphId::new(9), vec![GlyphId::new(12), GlyphId::new(4)]),
            (GlyphId::new(3), vec![GlyphId::new(8)]),
        ]);
        assert_eq!(
            substitutions.alternates(GlyphId::new(9)),
            Some(&[GlyphId::new(12), GlyphId::new(4),][..])
        );
        assert_eq!(substitutions.alternates(GlyphId::new(8)), None);
        assert_eq!(
            substitutions
                .mappings()
                .map(|(glyph, _)| glyph)
                .collect::<Vec<_>>(),
            [GlyphId::new(3), GlyphId::new(9),]
        );
        assert_eq!(substitutions.len(), 2);
        assert!(!substitutions.is_empty());
        assert!(AlternateSubstitution::default().is_empty());
    }
}

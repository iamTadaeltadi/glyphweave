//! Canonical one-to-many glyph substitutions.

use std::collections::BTreeMap;

use glyphweave_sfnt::GlyphId;

/// An input-ordered map replacing one glyph with an ordered glyph sequence.
///
/// An empty output sequence is retained because GSUB uses it to delete the input glyph.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MultipleSubstitution {
    mappings: BTreeMap<GlyphId, Vec<GlyphId>>,
}

impl MultipleSubstitution {
    /// Builds canonical mappings; a later duplicate input replaces an earlier sequence.
    #[must_use]
    pub fn from_mappings(mappings: impl IntoIterator<Item = (GlyphId, Vec<GlyphId>)>) -> Self {
        Self {
            mappings: mappings.into_iter().collect(),
        }
    }

    /// Returns the ordered replacement sequence, including a present empty deletion sequence.
    #[must_use]
    pub fn replacement(&self, input: GlyphId) -> Option<&[GlyphId]> {
        self.mappings.get(&input).map(Vec::as_slice)
    }

    /// Iterates mappings in ascending input Glyph ID order.
    pub fn mappings(&self) -> impl ExactSizeIterator<Item = (GlyphId, &[GlyphId])> + '_ {
        self.mappings
            .iter()
            .map(|(&input, output)| (input, output.as_slice()))
    }

    /// Returns the number of distinct input glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether no input glyph has a replacement sequence.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::MultipleSubstitution;

    #[test]
    fn preserves_sequence_order_and_explicit_deletion() {
        let substitutions = MultipleSubstitution::from_mappings([
            (GlyphId::new(9), vec![GlyphId::new(12), GlyphId::new(4)]),
            (GlyphId::new(3), Vec::new()),
        ]);
        assert_eq!(
            substitutions.replacement(GlyphId::new(9)),
            Some(&[GlyphId::new(12), GlyphId::new(4),][..])
        );
        assert_eq!(substitutions.replacement(GlyphId::new(3)), Some(&[][..]));
        assert_eq!(substitutions.replacement(GlyphId::new(8)), None);
        assert_eq!(substitutions.len(), 2);
        assert!(!substitutions.is_empty());
        assert!(MultipleSubstitution::default().is_empty());
    }
}

//! Resolved Unicode scalar to glyph mappings.

use std::collections::BTreeMap;

use crate::GlyphId;

/// A canonical resolved relation from Unicode scalar values to input Glyph IDs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterMap {
    mappings: BTreeMap<char, GlyphId>,
}

impl CharacterMap {
    /// Creates a Character Map with every scalar resolving to `.notdef`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mappings: BTreeMap::new(),
        }
    }

    /// Returns the number of explicit non-`.notdef` mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.mappings.len()
    }

    /// Returns whether the map has no explicit non-`.notdef` mappings.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    /// Resolves a Unicode scalar value, returning `.notdef` when it is unsupported.
    #[must_use]
    pub fn glyph_id(&self, codepoint: char) -> GlyphId {
        self.mappings
            .get(&codepoint)
            .copied()
            .unwrap_or(GlyphId::NOTDEF)
    }

    /// Visits explicit mappings in ascending Unicode scalar order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (char, GlyphId)> + DoubleEndedIterator + '_ {
        self.mappings
            .iter()
            .map(|(&codepoint, &glyph)| (codepoint, glyph))
    }
}

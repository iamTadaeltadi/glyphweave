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

    /// Assigns an input Glyph ID to a Unicode scalar value.
    ///
    /// Assigning `.notdef` removes any explicit entry because missing mappings already resolve to
    /// glyph zero. Returns the previous explicit non-`.notdef` mapping, if one existed.
    pub fn insert(&mut self, codepoint: char, glyph: GlyphId) -> Option<GlyphId> {
        if glyph.is_notdef() {
            self.mappings.remove(&codepoint)
        } else {
            self.mappings.insert(codepoint, glyph)
        }
    }

    /// Removes and returns an explicit mapping.
    pub fn remove(&mut self, codepoint: char) -> Option<GlyphId> {
        self.mappings.remove(&codepoint)
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

#[cfg(test)]
mod tests;

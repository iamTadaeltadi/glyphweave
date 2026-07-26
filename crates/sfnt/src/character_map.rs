//! Resolved Unicode scalar to glyph mappings.

use std::collections::BTreeMap;

use crate::GlyphId;

/// A canonical resolved relation from Unicode scalar values to input Glyph IDs.
///
/// ```
/// use glyphweave_sfnt::{CharacterMap, GlyphId};
///
/// let map: CharacterMap = [('A', GlyphId::new(7))].into_iter().collect();
/// assert_eq!(map.glyph_id('A'), GlyphId::new(7));
/// assert_eq!(map.glyph_id('B'), GlyphId::NOTDEF);
/// ```
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

    /// Retains only explicit mappings accepted by `predicate`.
    pub fn retain(&mut self, mut predicate: impl FnMut(char, GlyphId) -> bool) {
        self.mappings
            .retain(|&codepoint, glyph| predicate(codepoint, *glyph));
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

impl Extend<(char, GlyphId)> for CharacterMap {
    fn extend<T: IntoIterator<Item = (char, GlyphId)>>(&mut self, mappings: T) {
        for (codepoint, glyph) in mappings {
            self.insert(codepoint, glyph);
        }
    }
}

impl FromIterator<(char, GlyphId)> for CharacterMap {
    fn from_iter<T: IntoIterator<Item = (char, GlyphId)>>(mappings: T) -> Self {
        let mut map = Self::new();
        map.extend(mappings);
        map
    }
}

#[cfg(test)]
mod tests;

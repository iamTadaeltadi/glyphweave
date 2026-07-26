//! Deterministic retained-glyph membership.

use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

/// The unique input Glyph IDs retained by a subset operation.
///
/// A Glyph Set is never empty: [`GlyphId::NOTDEF`] is inserted at construction and cannot be
/// removed through the public API. Iteration follows ascending input Glyph ID order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphSet {
    glyphs: BTreeSet<GlyphId>,
}

impl GlyphSet {
    /// Creates the smallest valid Glyph Set, containing only `.notdef`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            glyphs: BTreeSet::from([GlyphId::NOTDEF]),
        }
    }

    /// Returns the number of retained Glyph IDs, including `.notdef`.
    #[must_use]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns whether no glyphs are retained.
    ///
    /// This is always `false` because every Glyph Set retains `.notdef`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns whether an input Glyph ID is retained.
    #[must_use]
    pub fn contains(&self, glyph: GlyphId) -> bool {
        self.glyphs.contains(&glyph)
    }

    /// Retains an input Glyph ID.
    ///
    /// Returns `true` when the glyph was newly inserted.
    pub fn insert(&mut self, glyph: GlyphId) -> bool {
        self.glyphs.insert(glyph)
    }

    /// Stops retaining a non-`.notdef` Glyph ID.
    ///
    /// Returns `true` when a member was removed. `.notdef` is never removed.
    pub fn remove(&mut self, glyph: GlyphId) -> bool {
        !glyph.is_notdef() && self.glyphs.remove(&glyph)
    }

    /// Retains only Glyph IDs accepted by `predicate`, plus mandatory `.notdef`.
    pub fn retain(&mut self, mut predicate: impl FnMut(GlyphId) -> bool) {
        self.glyphs
            .retain(|glyph| glyph.is_notdef() || predicate(*glyph));
    }

    /// Visits retained Glyph IDs in ascending order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = GlyphId> + DoubleEndedIterator + '_ {
        self.glyphs.iter().copied()
    }
}

impl Default for GlyphSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Extend<GlyphId> for GlyphSet {
    fn extend<T: IntoIterator<Item = GlyphId>>(&mut self, glyphs: T) {
        self.glyphs.extend(glyphs);
    }
}

impl FromIterator<GlyphId> for GlyphSet {
    fn from_iter<T: IntoIterator<Item = GlyphId>>(glyphs: T) -> Self {
        let mut set = Self::new();
        set.extend(glyphs);
        set
    }
}

impl<'a> IntoIterator for &'a GlyphSet {
    type Item = GlyphId;
    type IntoIter = std::iter::Copied<std::collections::btree_set::Iter<'a, GlyphId>>;

    fn into_iter(self) -> Self::IntoIter {
        self.glyphs.iter().copied()
    }
}

#[cfg(test)]
mod tests;

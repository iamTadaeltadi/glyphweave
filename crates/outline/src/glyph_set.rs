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

impl<'a> IntoIterator for &'a GlyphSet {
    type Item = GlyphId;
    type IntoIter = std::iter::Copied<std::collections::btree_set::Iter<'a, GlyphId>>;

    fn into_iter(self) -> Self::IntoIter {
        self.glyphs.iter().copied()
    }
}

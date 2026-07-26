//! Fixed-point GSUB glyph and nested-Lookup reachability.

use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

/// The monotone fixed point reached from selected glyphs and root GSUB Lookups.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GsubClosure {
    pub(super) glyphs: BTreeSet<GlyphId>,
    pub(super) lookups: BTreeSet<u16>,
    pub(super) passes: usize,
}

impl GsubClosure {
    /// Returns whether a glyph belongs to the substitution closure.
    #[must_use]
    pub fn contains(&self, glyph: GlyphId) -> bool {
        self.glyphs.contains(&glyph)
    }

    /// Iterates closed glyphs in ascending input Glyph ID order.
    pub fn glyphs(&self) -> impl ExactSizeIterator<Item = GlyphId> + DoubleEndedIterator + '_ {
        self.glyphs.iter().copied()
    }

    /// Returns whether an original LookupList index became reachable.
    #[must_use]
    pub fn contains_lookup(&self, index: u16) -> bool {
        self.lookups.contains(&index)
    }

    /// Iterates reachable original LookupList indices in ascending order.
    pub fn lookup_indices(&self) -> impl ExactSizeIterator<Item = u16> + DoubleEndedIterator + '_ {
        self.lookups.iter().copied()
    }

    /// Returns the number of growth passes needed to prove the fixed point.
    #[must_use]
    pub const fn passes(&self) -> usize {
        self.passes
    }
}

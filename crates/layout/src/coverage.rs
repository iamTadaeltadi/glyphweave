//! Canonical glyph coverage and Coverage Index lookup.

use glyphweave_sfnt::GlyphId;

mod decode;
mod encode;

/// The sorted unique glyphs affected by an OpenType layout operation.
///
/// Coverage Indices are implicit positions in this sequence and therefore remain contiguous.
///
/// ```
/// use glyphweave_layout::Coverage;
/// use glyphweave_sfnt::GlyphId;
///
/// let coverage = Coverage::from_glyphs([GlyphId::new(8), GlyphId::new(3), GlyphId::new(8)]);
/// assert_eq!(coverage.glyphs(), &[GlyphId::new(3), GlyphId::new(8)]);
/// assert_eq!(coverage.coverage_index(GlyphId::new(8)), Some(1));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Coverage<G = GlyphId> {
    glyphs: Vec<G>,
}

impl<G: Copy + Ord> Coverage<G> {
    /// Builds canonical Coverage from arbitrary input order, removing duplicates.
    #[must_use]
    pub fn from_glyphs(glyphs: impl IntoIterator<Item = G>) -> Self {
        let mut glyphs: Vec<_> = glyphs.into_iter().collect();
        glyphs.sort_unstable();
        glyphs.dedup();
        Self { glyphs }
    }

    /// Returns the covered glyphs in ascending Glyph ID order.
    #[must_use]
    pub fn glyphs(&self) -> &[G] {
        &self.glyphs
    }

    /// Returns the number of covered glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Returns whether no glyph is covered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.glyphs.is_empty()
    }

    /// Resolves a Glyph ID to its sequential Coverage Index.
    #[must_use]
    pub fn coverage_index(&self, glyph: G) -> Option<u16> {
        self.glyphs
            .binary_search(&glyph)
            .ok()
            .and_then(|index| u16::try_from(index).ok())
    }

    /// Resolves a Coverage Index to its Glyph ID.
    #[must_use]
    pub fn glyph_at(&self, coverage_index: u16) -> Option<G> {
        self.glyphs.get(usize::from(coverage_index)).copied()
    }
}

impl<G: Copy + Ord> FromIterator<G> for Coverage<G> {
    fn from_iter<T: IntoIterator<Item = G>>(iter: T) -> Self {
        Self::from_glyphs(iter)
    }
}

#[cfg(test)]
mod tests;

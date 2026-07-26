//! Canonical glyph coverage and Coverage Index lookup.

use glyphweave_sfnt::GlyphId;

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
pub struct Coverage {
    glyphs: Vec<GlyphId>,
}

impl Coverage {
    /// Builds canonical Coverage from arbitrary input order, removing duplicates.
    #[must_use]
    pub fn from_glyphs(glyphs: impl IntoIterator<Item = GlyphId>) -> Self {
        let mut glyphs: Vec<_> = glyphs.into_iter().collect();
        glyphs.sort_unstable();
        glyphs.dedup();
        Self { glyphs }
    }

    /// Returns the covered glyphs in ascending Glyph ID order.
    #[must_use]
    pub fn glyphs(&self) -> &[GlyphId] {
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
    pub fn coverage_index(&self, glyph: GlyphId) -> Option<u16> {
        self.glyphs
            .binary_search(&glyph)
            .ok()
            .and_then(|index| u16::try_from(index).ok())
    }

    /// Resolves a Coverage Index to its Glyph ID.
    #[must_use]
    pub fn glyph_at(&self, coverage_index: u16) -> Option<GlyphId> {
        self.glyphs.get(usize::from(coverage_index)).copied()
    }
}

impl FromIterator<GlyphId> for Coverage {
    fn from_iter<T: IntoIterator<Item = GlyphId>>(iter: T) -> Self {
        Self::from_glyphs(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::Coverage;
    use glyphweave_sfnt::GlyphId;

    #[test]
    fn default_coverage_is_empty() {
        let coverage = Coverage::default();
        assert!(coverage.is_empty());
        assert_eq!(coverage.len(), 0);
        assert!(coverage.glyphs().is_empty());
    }

    #[test]
    fn construction_sorts_and_deduplicates_glyphs() {
        let coverage = Coverage::from_glyphs([
            GlyphId::new(7),
            GlyphId::new(2),
            GlyphId::new(7),
            GlyphId::NOTDEF,
        ]);
        assert_eq!(
            coverage.glyphs(),
            &[GlyphId::NOTDEF, GlyphId::new(2), GlyphId::new(7)]
        );
    }

    #[test]
    fn lookup_is_bidirectional_for_present_glyphs() {
        let coverage = Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(9)]);
        assert_eq!(coverage.coverage_index(GlyphId::new(4)), Some(0));
        assert_eq!(coverage.coverage_index(GlyphId::new(9)), Some(1));
        assert_eq!(coverage.glyph_at(0), Some(GlyphId::new(4)));
        assert_eq!(coverage.glyph_at(1), Some(GlyphId::new(9)));
    }

    #[test]
    fn lookup_rejects_absent_glyphs_and_indices() {
        let coverage = Coverage::from_glyphs([GlyphId::new(4)]);
        assert_eq!(coverage.coverage_index(GlyphId::new(3)), None);
        assert_eq!(coverage.glyph_at(1), None);
    }
}

//! Cycle-safe transitive retention of composite components.

use glyphweave_sfnt::GlyphId;

use crate::GlyphSet;

/// A closed Glyph Set and its deepest reachable composite-component path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphClosure {
    pub(crate) glyphs: GlyphSet,
    pub(crate) maximum_component_depth: u16,
}

impl GlyphClosure {
    /// Returns the greatest number of component edges from a retained root to a leaf.
    #[must_use]
    pub const fn maximum_component_depth(&self) -> u16 {
        self.maximum_component_depth
    }

    /// Returns whether a Glyph ID belongs to the transitive closure.
    #[must_use]
    pub fn contains(&self, glyph: GlyphId) -> bool {
        self.glyphs.contains(glyph)
    }

    /// Returns the number of retained Glyph IDs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.glyphs.len()
    }

    /// Borrows the complete retained Glyph Set.
    #[must_use]
    pub const fn glyphs(&self) -> &GlyphSet {
        &self.glyphs
    }
}

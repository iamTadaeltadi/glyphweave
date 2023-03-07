//! Eagerly decoded TrueType outline collection.

use glyphweave_sfnt::{ByteSpan, GlyphId};

use crate::Outline;

/// Canonical outlines indexed by input Glyph ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glyf {
    pub(crate) table_span: ByteSpan,
    pub(crate) outlines: Vec<Outline>,
}

impl Glyf {
    /// Returns canonical outlines in ascending input Glyph ID order.
    #[must_use]
    pub fn outlines(&self) -> &[Outline] {
        &self.outlines
    }

    /// Borrows the canonical outline for an in-range input Glyph ID.
    #[must_use]
    pub fn outline(&self, glyph: GlyphId) -> Option<&Outline> {
        self.outlines.get(usize::from(glyph.raw()))
    }

    /// Returns the number of input Glyph IDs represented by the collection.
    #[must_use]
    pub fn glyph_count(&self) -> usize {
        self.outlines.len()
    }

    /// Returns the checked span occupied by the source `glyf` table.
    #[must_use]
    pub const fn table_span(&self) -> ByteSpan {
        self.table_span
    }
}

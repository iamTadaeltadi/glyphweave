//! Eagerly decoded TrueType outline collection.

use glyphweave_sfnt::ByteSpan;

use crate::Outline;

/// Canonical outlines indexed by input Glyph ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glyf {
    pub(crate) table_span: ByteSpan,
    pub(crate) outlines: Vec<Outline>,
}

impl Glyf {
    /// Returns the checked span occupied by the source `glyf` table.
    #[must_use]
    pub const fn table_span(&self) -> ByteSpan {
        self.table_span
    }
}

//! Eagerly decoded TrueType outline collection.

use glyphweave_sfnt::ByteSpan;

use crate::Outline;

/// Canonical outlines indexed by input Glyph ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glyf {
    pub(crate) table_span: ByteSpan,
    pub(crate) outlines: Vec<Outline>,
}

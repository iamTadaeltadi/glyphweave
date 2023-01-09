//! Canonical child sequence for one TrueType composite outline.

use glyphweave_sfnt::ByteSpan;

use crate::{Component, GlyphBounds};

/// A decoded composite glyph whose child graph remains unresolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompositeGlyph {
    pub(crate) span: ByteSpan,
    pub(crate) bounds: GlyphBounds,
    pub(crate) components: Vec<Component>,
    pub(crate) instructions: Vec<u8>,
    pub(crate) overlapping_components: bool,
}

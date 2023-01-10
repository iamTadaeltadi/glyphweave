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

impl CompositeGlyph {
    /// Returns the complete checked Glyph Byte Span in `glyf`.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        self.span
    }

    /// Returns the recorded bounds of the accumulated child outline.
    #[must_use]
    pub const fn bounds(&self) -> GlyphBounds {
        self.bounds
    }
}

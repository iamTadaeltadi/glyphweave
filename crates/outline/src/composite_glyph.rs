//! Canonical child sequence for one TrueType composite outline.

mod decode;

use glyphweave_sfnt::{ByteSpan, Decoded};

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
    /// Decodes one non-empty composite Glyph Byte Span from `glyf`.
    ///
    /// ```
    /// use glyphweave_outline::CompositeGlyph;
    /// use glyphweave_sfnt::ByteSpan;
    ///
    /// let bytes = [
    ///     0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0,
    ///     0, 2, 0, 0, 0, 0,
    /// ];
    /// let span = ByteSpan::sized(0, bytes.len()).unwrap();
    /// let glyph = CompositeGlyph::decode(&bytes, span)?;
    /// assert_eq!(glyph.component_count(), 1);
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when the header describes a simple glyph or any component,
    /// transform, instruction, flag, or alignment invariant is malformed.
    pub fn decode(bytes: &[u8], glyph_span: ByteSpan) -> Decoded<Self> {
        decode::decode(bytes, glyph_span)
    }

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

    /// Returns child components in source order.
    #[must_use]
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Returns the number of direct child components.
    #[must_use]
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Returns the uninterpreted instruction program for the accumulated outline.
    #[must_use]
    pub fn instructions(&self) -> &[u8] {
        &self.instructions
    }

    /// Returns whether the source marks that component outlines may overlap.
    #[must_use]
    pub const fn has_overlapping_components(&self) -> bool {
        self.overlapping_components
    }
}

#[cfg(test)]
mod tests;

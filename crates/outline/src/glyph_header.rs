//! The common semantic header shared by simple and composite glyphs.

mod decode;

use glyphweave_sfnt::{ByteSpan, Decoded};

use crate::GlyphBounds;

/// A classified TrueType Glyph Header with ordered bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlyphHeader {
    number_of_contours: i16,
    bounds: GlyphBounds,
}

impl GlyphHeader {
    /// Decodes the common header at the start of one non-empty Glyph Byte Span.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when the span is empty, out of bounds, shorter than the
    /// ten-byte header, or records an inverted bounding axis.
    pub fn decode(bytes: &[u8], glyph_span: ByteSpan) -> Decoded<Self> {
        decode::decode(bytes, glyph_span)
    }

    /// Constructs a header from its signed contour marker and validated bounds.
    #[must_use]
    pub const fn new(number_of_contours: i16, bounds: GlyphBounds) -> Self {
        Self {
            number_of_contours,
            bounds,
        }
    }

    /// Returns the raw signed contour count from `glyf`.
    #[must_use]
    pub const fn number_of_contours(self) -> i16 {
        self.number_of_contours
    }

    /// Returns the nonnegative contour count for a simple glyph.
    #[must_use]
    pub fn simple_contour_count(self) -> Option<u16> {
        u16::try_from(self.number_of_contours).ok()
    }

    /// Returns whether a negative contour marker classifies a composite glyph.
    #[must_use]
    pub const fn is_composite(self) -> bool {
        self.number_of_contours < 0
    }

    /// Returns the ordered control-point bounds.
    #[must_use]
    pub const fn bounds(self) -> GlyphBounds {
        self.bounds
    }
}

#[cfg(test)]
mod tests;

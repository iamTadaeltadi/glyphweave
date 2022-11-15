//! Canonical simple TrueType outlines.

use glyphweave_sfnt::ByteSpan;

use crate::{GlyphBounds, Point};

/// A decoded simple glyph with absolute points and exclusive contour ends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleGlyph {
    pub(crate) span: ByteSpan,
    pub(crate) bounds: GlyphBounds,
    pub(crate) contour_ends: Vec<u32>,
    pub(crate) points: Vec<Point>,
    pub(crate) instructions: Vec<u8>,
    pub(crate) overlapping_contours: bool,
}

impl SimpleGlyph {
    /// Returns the complete checked Glyph Byte Span in `glyf`.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        self.span
    }

    /// Returns the bounds recorded and verified against all control points.
    #[must_use]
    pub const fn bounds(&self) -> GlyphBounds {
        self.bounds
    }

    /// Returns the number of contours in the glyph.
    #[must_use]
    pub fn contour_count(&self) -> usize {
        self.contour_ends.len()
    }

    /// Returns exclusive point indices delimiting the contours.
    #[must_use]
    pub fn contour_ends(&self) -> &[u32] {
        &self.contour_ends
    }

    /// Returns all points in contour order.
    #[must_use]
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Returns one contour's points when `index` is in range.
    #[must_use]
    pub fn contour(&self, index: usize) -> Option<&[Point]> {
        let end = usize::try_from(*self.contour_ends.get(index)?).ok()?;
        let start = if index == 0 {
            0
        } else {
            usize::try_from(*self.contour_ends.get(index - 1)?).ok()?
        };
        self.points.get(start..end)
    }

    /// Returns the uninterpreted TrueType instruction program.
    #[must_use]
    pub fn instructions(&self) -> &[u8] {
        &self.instructions
    }

    /// Returns whether the source marked that simple contours may overlap.
    #[must_use]
    pub const fn has_overlapping_contours(&self) -> bool {
        self.overlapping_contours
    }
}

#[cfg(test)]
mod tests;

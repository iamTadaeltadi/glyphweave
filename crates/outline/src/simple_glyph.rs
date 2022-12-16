//! Canonical simple TrueType outlines.

mod decode;

use glyphweave_sfnt::{ByteSpan, Decoded};

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
    /// Decodes one non-empty simple Glyph Byte Span from `glyf`.
    ///
    /// ```
    /// use glyphweave_outline::SimpleGlyph;
    /// use glyphweave_sfnt::ByteSpan;
    ///
    /// let bytes = [0; 10];
    /// let span = ByteSpan::sized(0, bytes.len()).unwrap();
    /// let glyph = SimpleGlyph::decode(&bytes, span)?;
    /// assert_eq!(glyph.contour_count(), 0);
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when the header describes a composite glyph or any contour,
    /// instruction, packed flag, coordinate, bounds, or alignment invariant is malformed.
    pub fn decode(bytes: &[u8], glyph_span: ByteSpan) -> Decoded<Self> {
        decode::decode(bytes, glyph_span)
    }

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

    /// Returns the total number of logical control points across all contours.
    #[must_use]
    pub fn point_count(&self) -> usize {
        self.points.len()
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

    /// Iterates over contour point slices in source order.
    #[must_use]
    pub fn contours(&self) -> impl ExactSizeIterator<Item = &[Point]> {
        let mut start = 0usize;
        self.contour_ends.iter().map(move |&end| {
            let end = usize::try_from(end).unwrap_or(self.points.len());
            let contour = self.points.get(start..end).unwrap_or(&[]);
            start = end;
            contour
        })
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

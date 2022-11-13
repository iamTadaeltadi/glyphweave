//! Ordered control-point bounds from a TrueType Glyph Header.

/// The inclusive design-unit bounds recorded by a TrueType glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GlyphBounds {
    minimum_x: i16,
    minimum_y: i16,
    maximum_x: i16,
    maximum_y: i16,
}

impl GlyphBounds {
    /// Constructs bounds whose minima do not exceed their maxima.
    #[must_use]
    pub const fn new(
        minimum_x: i16,
        minimum_y: i16,
        maximum_x: i16,
        maximum_y: i16,
    ) -> Option<Self> {
        if minimum_x <= maximum_x && minimum_y <= maximum_y {
            Some(Self {
                minimum_x,
                minimum_y,
                maximum_x,
                maximum_y,
            })
        } else {
            None
        }
    }

    /// Returns the left edge in design units.
    #[must_use]
    pub const fn minimum_x(self) -> i16 {
        self.minimum_x
    }

    /// Returns the bottom edge in design units.
    #[must_use]
    pub const fn minimum_y(self) -> i16 {
        self.minimum_y
    }

    /// Returns the right edge in design units.
    #[must_use]
    pub const fn maximum_x(self) -> i16 {
        self.maximum_x
    }

    /// Returns the top edge in design units.
    #[must_use]
    pub const fn maximum_y(self) -> i16 {
        self.maximum_y
    }
}

#[cfg(test)]
mod tests;

//! Canonical absolute TrueType control points.

/// One on-curve or off-curve control point in font design units.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i16,
    y: i16,
    on_curve: bool,
}

impl Point {
    /// Constructs an absolute control point.
    #[must_use]
    pub const fn new(x: i16, y: i16, on_curve: bool) -> Self {
        Self { x, y, on_curve }
    }

    /// Returns the absolute horizontal coordinate in design units.
    #[must_use]
    pub const fn x(self) -> i16 {
        self.x
    }

    /// Returns the absolute vertical coordinate in design units.
    #[must_use]
    pub const fn y(self) -> i16 {
        self.y
    }

    /// Returns whether the point lies on the quadratic curve.
    #[must_use]
    pub const fn is_on_curve(self) -> bool {
        self.on_curve
    }
}

#[cfg(test)]
mod tests;

//! Canonical OpenType GPOS anchor attachment points.

mod decode;
#[cfg(test)]
mod tests;

/// A GPOS anchor attachment point in glyph design space.
///
/// Every anchor carries an `(x, y)` design-space coordinate. Format 2 additionally names a
/// contour point whose fitted outline position overrides the coordinate when the glyph is
/// hinted, and format 3 attaches per-axis device or variation-index tables that refine the
/// coordinate. This subsetter retains the coordinate and records which refinement is present;
/// format 3 device offsets are bounds-checked during decoding but deliberately left unresolved.
///
/// ```
/// use glyphweave_layout::{Anchor, AnchorKind};
///
/// let anchor = Anchor::coordinate(120, -40);
/// assert_eq!((anchor.x(), anchor.y()), (120, -40));
/// assert_eq!(anchor.contour_point_index(), None);
/// assert!(!anchor.has_devices());
/// assert_eq!(anchor.kind(), AnchorKind::Coordinate);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Anchor {
    x: i16,
    y: i16,
    kind: AnchorKind,
}

/// The format-specific refinement an [`Anchor`] carries beyond its base coordinate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AnchorKind {
    /// Format 1: the design-space coordinate stands alone.
    Coordinate,
    /// Format 2: a contour point index whose fitted position overrides the coordinate.
    ContourPoint(u16),
    /// Format 3: per-axis device or variation-index table offsets, validated but not resolved.
    Devices {
        /// The horizontal device offset relative to the Anchor table, or `None` when null.
        x_device: Option<u16>,
        /// The vertical device offset relative to the Anchor table, or `None` when null.
        y_device: Option<u16>,
    },
}

impl Anchor {
    /// Builds a format 1 anchor from a lone design-space coordinate.
    #[must_use]
    pub const fn coordinate(x: i16, y: i16) -> Self {
        Self {
            x,
            y,
            kind: AnchorKind::Coordinate,
        }
    }

    /// Builds a format 2 anchor whose coordinate is refined by a contour point index.
    #[must_use]
    pub const fn contour_point(x: i16, y: i16, contour_point: u16) -> Self {
        Self {
            x,
            y,
            kind: AnchorKind::ContourPoint(contour_point),
        }
    }

    /// Builds a format 3 anchor carrying optional per-axis device offsets.
    #[must_use]
    pub const fn devices(x: i16, y: i16, x_device: Option<u16>, y_device: Option<u16>) -> Self {
        Self {
            x,
            y,
            kind: AnchorKind::Devices { x_device, y_device },
        }
    }

    /// The horizontal design-space coordinate.
    #[must_use]
    pub const fn x(self) -> i16 {
        self.x
    }

    /// The vertical design-space coordinate.
    #[must_use]
    pub const fn y(self) -> i16 {
        self.y
    }

    /// The format-specific refinement carried alongside the coordinate.
    #[must_use]
    pub const fn kind(self) -> AnchorKind {
        self.kind
    }

    /// Returns the format 2 contour point index, or `None` for other formats.
    #[must_use]
    pub const fn contour_point_index(self) -> Option<u16> {
        match self.kind {
            AnchorKind::ContourPoint(point) => Some(point),
            AnchorKind::Coordinate | AnchorKind::Devices { .. } => None,
        }
    }

    /// Returns whether this anchor references at least one non-null device table.
    #[must_use]
    pub const fn has_devices(self) -> bool {
        match self.kind {
            AnchorKind::Devices { x_device, y_device } => x_device.is_some() || y_device.is_some(),
            AnchorKind::Coordinate | AnchorKind::ContourPoint(_) => false,
        }
    }
}

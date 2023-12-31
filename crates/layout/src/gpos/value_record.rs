//! Canonical OpenType GPOS positioning value records.

/// A glyph-positioning adjustment: optional design-space placement and advance deltas.
///
/// A GPOS value record selects its present fields through a value-format bitmask. This type
/// holds the four integer adjustments a subset preserves unchanged; device and variation-index
/// tables carry no Glyph ID and are resolved by the positioning subtable decoders, not here.
///
/// ```
/// use glyphweave_layout::ValueRecord;
///
/// let advance = ValueRecord::new(0, 0, -40, 0);
/// assert_eq!(advance.x_advance(), -40);
/// assert!(!advance.is_zero());
/// assert!(ValueRecord::default().is_zero());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValueRecord {
    x_placement: i16,
    y_placement: i16,
    x_advance: i16,
    y_advance: i16,
}

impl ValueRecord {
    /// Builds a value record from its four design-space adjustments.
    #[must_use]
    pub const fn new(x_placement: i16, y_placement: i16, x_advance: i16, y_advance: i16) -> Self {
        Self {
            x_placement,
            y_placement,
            x_advance,
            y_advance,
        }
    }

    /// The horizontal placement adjustment.
    #[must_use]
    pub const fn x_placement(self) -> i16 {
        self.x_placement
    }

    /// The vertical placement adjustment.
    #[must_use]
    pub const fn y_placement(self) -> i16 {
        self.y_placement
    }

    /// The horizontal advance adjustment.
    #[must_use]
    pub const fn x_advance(self) -> i16 {
        self.x_advance
    }

    /// The vertical advance adjustment.
    #[must_use]
    pub const fn y_advance(self) -> i16 {
        self.y_advance
    }

    /// Returns whether every adjustment is zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.x_placement == 0 && self.y_placement == 0 && self.x_advance == 0 && self.y_advance == 0
    }
}

#[cfg(test)]
mod tests;

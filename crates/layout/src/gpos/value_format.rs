//! GPOS value-format bitmasks selecting which value-record fields are present.

/// The bitmask that selects which adjustments and device tables a value record encodes.
///
/// The OpenType value format reserves the high byte; a format with any reserved bit set is
/// rejected at construction so later decoding can trust every recognized flag.
///
/// ```
/// use glyphweave_layout::ValueFormat;
///
/// let format = ValueFormat::from_bits(0x0005).expect("x placement + x advance");
/// assert!(format.has_x_placement());
/// assert!(format.has_x_advance());
/// assert!(!format.has_y_placement());
/// assert_eq!(format.byte_size(), 4);
/// assert!(ValueFormat::from_bits(0x0100).is_none());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ValueFormat(u16);

impl ValueFormat {
    const X_PLACEMENT: u16 = 0x0001;
    const Y_PLACEMENT: u16 = 0x0002;
    const X_ADVANCE: u16 = 0x0004;
    const Y_ADVANCE: u16 = 0x0008;
    const X_PLACEMENT_DEVICE: u16 = 0x0010;
    const Y_PLACEMENT_DEVICE: u16 = 0x0020;
    const X_ADVANCE_DEVICE: u16 = 0x0040;
    const Y_ADVANCE_DEVICE: u16 = 0x0080;
    const RECOGNIZED: u16 = 0x00ff;

    /// Builds a value format, rejecting any format that sets a reserved bit.
    #[must_use]
    pub const fn from_bits(bits: u16) -> Option<Self> {
        if bits & !Self::RECOGNIZED == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }

    /// Returns whether the format encodes a horizontal placement adjustment.
    #[must_use]
    pub const fn has_x_placement(self) -> bool {
        self.0 & Self::X_PLACEMENT != 0
    }

    /// Returns whether the format encodes a vertical placement adjustment.
    #[must_use]
    pub const fn has_y_placement(self) -> bool {
        self.0 & Self::Y_PLACEMENT != 0
    }

    /// Returns whether the format encodes a horizontal advance adjustment.
    #[must_use]
    pub const fn has_x_advance(self) -> bool {
        self.0 & Self::X_ADVANCE != 0
    }

    /// Returns whether the format encodes a vertical advance adjustment.
    #[must_use]
    pub const fn has_y_advance(self) -> bool {
        self.0 & Self::Y_ADVANCE != 0
    }

    /// Returns whether the format references any device or variation-index table.
    #[must_use]
    pub const fn has_any_device(self) -> bool {
        let devices = Self::X_PLACEMENT_DEVICE
            | Self::Y_PLACEMENT_DEVICE
            | Self::X_ADVANCE_DEVICE
            | Self::Y_ADVANCE_DEVICE;
        self.0 & devices != 0
    }

    /// Returns the encoded size of one value record in bytes: two bytes per set flag.
    #[must_use]
    pub const fn byte_size(self) -> usize {
        self.0.count_ones() as usize * 2
    }

    /// Returns whether the format encodes no fields.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests;

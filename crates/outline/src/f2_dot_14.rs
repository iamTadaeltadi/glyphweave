//! Exact signed 2.14 fixed-point values used by component transforms.

/// A signed two-integer-bit, fourteen-fractional-bit transform coefficient.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct F2Dot14(i16);

impl F2Dot14 {
    /// The exact fixed-point representation of zero.
    pub const ZERO: Self = Self(0);

    /// The exact fixed-point representation of one.
    pub const ONE: Self = Self(0x4000);

    /// Preserves an encoded F2DOT14 bit pattern.
    #[must_use]
    pub const fn from_bits(bits: i16) -> Self {
        Self(bits)
    }

    /// Returns the original signed fixed-point bits.
    #[must_use]
    pub const fn to_bits(self) -> i16 {
        self.0
    }

    /// Converts the exact fixed-point value to a double-precision number.
    #[must_use]
    pub fn to_f64(self) -> f64 {
        f64::from(self.0) / 16_384.0
    }
}

#[cfg(test)]
mod tests;

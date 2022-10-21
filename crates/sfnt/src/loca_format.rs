//! The closed `head.indexToLocFormat` domain.

/// The entry width used by a TrueType `loca` table.
///
/// ```
/// use glyphweave_sfnt::LocaFormat;
///
/// assert_eq!(LocaFormat::new(0), Some(LocaFormat::Short));
/// assert_eq!(LocaFormat::new(1), Some(LocaFormat::Long));
/// assert_eq!(LocaFormat::new(2), None);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LocaFormat {
    /// Unsigned 16-bit entries storing byte offsets divided by two.
    Short,
    /// Unsigned 32-bit entries storing byte offsets directly.
    Long,
}

impl LocaFormat {
    /// Constructs the format encoded by `head.indexToLocFormat`.
    #[must_use]
    pub const fn new(value: i16) -> Option<Self> {
        match value {
            0 => Some(Self::Short),
            1 => Some(Self::Long),
            _ => None,
        }
    }

    /// Returns the signed value stored by `head.indexToLocFormat`.
    #[must_use]
    pub const fn value(self) -> i16 {
        match self {
            Self::Short => 0,
            Self::Long => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LocaFormat;

    #[test]
    fn accepts_both_standardized_values() {
        assert_eq!(LocaFormat::new(0), Some(LocaFormat::Short));
        assert_eq!(LocaFormat::new(1), Some(LocaFormat::Long));
    }

    #[test]
    fn rejects_every_adjacent_and_extreme_value() {
        for value in [i16::MIN, -1, 2, i16::MAX] {
            assert_eq!(LocaFormat::new(value), None);
        }
    }

    #[test]
    fn standardized_values_round_trip() {
        for format in [LocaFormat::Short, LocaFormat::Long] {
            assert_eq!(LocaFormat::new(format.value()), Some(format));
        }
    }
}

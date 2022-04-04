//! Enumerated SFNT versions used by a Table Directory.

/// The outline flavor declared by an SFNT Table Directory.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SfntVersion {
    /// TrueType outlines, encoded as fixed version 1.0 (`0x0001_0000`).
    TrueType,
    /// Compact Font Format outlines, encoded as the tag `OTTO`.
    OpenTypeCff,
}

impl SfntVersion {
    /// Decodes a standardized SFNT version value.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            0x0001_0000 => Some(Self::TrueType),
            0x4f54_544f => Some(Self::OpenTypeCff),
            _ => None,
        }
    }

    /// Returns the four bytes stored in the Table Directory.
    #[must_use]
    pub const fn raw(self) -> u32 {
        match self {
            Self::TrueType => 0x0001_0000,
            Self::OpenTypeCff => 0x4f54_544f,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SfntVersion;

    #[test]
    fn standardized_versions_round_trip() {
        for version in [SfntVersion::TrueType, SfntVersion::OpenTypeCff] {
            assert_eq!(SfntVersion::from_raw(version.raw()), Some(version));
        }
    }

    #[test]
    fn legacy_and_unknown_versions_are_not_opentype() {
        assert_eq!(SfntVersion::from_raw(u32::from_be_bytes(*b"true")), None);
        assert_eq!(SfntVersion::from_raw(0), None);
    }
}

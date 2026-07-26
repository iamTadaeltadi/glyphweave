//! Standardized TTC header versions.

/// A supported Font Collection header version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CollectionVersion {
    /// TTC Header version 1.0, without file-level signature fields.
    Version1,
    /// TTC Header version 2.0, with optional file-level signature fields.
    Version2,
}

impl CollectionVersion {
    pub(crate) const fn from_parts(major: u16, minor: u16) -> Option<Self> {
        match (major, minor) {
            (1, 0) => Some(Self::Version1),
            (2, 0) => Some(Self::Version2),
            _ => None,
        }
    }

    /// Returns the `(major, minor)` values stored in the TTC Header.
    #[must_use]
    pub const fn parts(self) -> (u16, u16) {
        match self {
            Self::Version1 => (1, 0),
            Self::Version2 => (2, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CollectionVersion;

    #[test]
    fn standardized_versions_round_trip() {
        for version in [CollectionVersion::Version1, CollectionVersion::Version2] {
            let (major, minor) = version.parts();
            assert_eq!(CollectionVersion::from_parts(major, minor), Some(version));
        }
    }

    #[test]
    fn extensions_and_unknown_major_versions_are_rejected() {
        assert_eq!(CollectionVersion::from_parts(1, 1), None);
        assert_eq!(CollectionVersion::from_parts(2, 1), None);
        assert_eq!(CollectionVersion::from_parts(3, 0), None);
    }
}

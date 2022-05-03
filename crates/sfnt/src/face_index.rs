//! A typed selector for one face in a font file or collection.

use core::fmt;

/// The zero-based position of a Face in a Font Collection.
///
/// Keeping the selector distinct from byte offsets prevents a caller from accidentally using a
/// directory location as a collection index.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceIndex(u32);

impl FaceIndex {
    /// The first and only valid index for a standalone SFNT font.
    pub const FIRST: Self = Self(0);

    /// Creates a Face Index from its zero-based integer representation.
    #[must_use]
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    /// Returns the zero-based integer representation.
    #[must_use]
    pub const fn raw(self) -> u32 {
        self.0
    }
}

impl fmt::Debug for FaceIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("FaceIndex").field(&self.0).finish()
    }
}

impl fmt::Display for FaceIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::FaceIndex;

    #[test]
    fn index_preserves_its_zero_based_value() {
        let index = FaceIndex::new(7);
        assert_eq!(index.raw(), 7);
        assert_eq!(index.to_string(), "7");
        assert_eq!(format!("{index:?}"), "FaceIndex(7)");
    }

    #[test]
    fn first_index_is_zero() {
        assert_eq!(FaceIndex::FIRST, FaceIndex::default());
    }
}

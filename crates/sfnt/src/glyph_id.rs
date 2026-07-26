//! Type-distinct input and emitted glyph indices.

use core::fmt;

/// A glyph index in an input font.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlyphId(u16);

impl GlyphId {
    /// The mandatory missing-glyph entry at input glyph index zero.
    pub const NOTDEF: Self = Self(0);

    /// Creates an input Glyph ID from its unsigned font representation.
    #[must_use]
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    /// Returns the unsigned value stored by the font.
    #[must_use]
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// Returns `true` for the mandatory `.notdef` glyph.
    #[must_use]
    pub const fn is_notdef(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Debug for GlyphId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_tuple("GlyphId").field(&self.0).finish()
    }
}

impl fmt::Display for GlyphId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::GlyphId;

    #[test]
    fn glyph_id_preserves_full_unsigned_range() {
        for raw in [0, 1, u16::MAX] {
            let glyph = GlyphId::new(raw);
            assert_eq!(glyph.raw(), raw);
            assert_eq!(glyph.to_string(), raw.to_string());
            assert_eq!(format!("{glyph:?}"), format!("GlyphId({raw})"));
        }
    }

    #[test]
    fn notdef_is_exactly_input_glyph_zero() {
        assert!(GlyphId::NOTDEF.is_notdef());
        assert_eq!(GlyphId::NOTDEF, GlyphId::default());
        assert!(!GlyphId::new(1).is_notdef());
    }
}

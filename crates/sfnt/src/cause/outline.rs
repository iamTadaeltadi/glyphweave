//! Typed causes produced while decoding TrueType outlines.

use core::fmt;

/// An outline-specific cause produced while decoding `loca` or `glyf` data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OutlineCause {
    /// A `loca` table length disagrees with its format and Glyph count.
    InvalidLocaLength {
        /// The entry format selected by `head.indexToLocFormat`.
        format: crate::LocaFormat,
        /// The number of glyphs declared by `maxp.numGlyphs`.
        glyph_count: u16,
        /// The exact required table length.
        expected: usize,
        /// The unpadded length declared by the `loca` Table Record.
        actual: usize,
    },
}

impl fmt::Display for OutlineCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidLocaLength {
                format,
                glyph_count,
                expected,
                actual,
            } => write!(
                f,
                "{format:?} loca length {actual} does not equal {expected} bytes required by {glyph_count} glyph(s)"
            ),
        }
    }
}

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
    /// The first `loca` entry leaves unreferenced bytes before glyph zero.
    NonzeroLocaStart {
        /// The first relative `glyf` offset.
        offset: u32,
    },
    /// A `loca` entry moves backward in `glyf`.
    DecreasingLocaOffset {
        /// The zero-based location-entry index.
        index: u32,
        /// The decreasing relative offset.
        offset: u32,
        /// The immediately preceding relative offset.
        previous: u32,
    },
    /// A long-format `loca` entry is not word aligned.
    MisalignedLocaOffset {
        /// The zero-based location-entry index.
        index: u32,
        /// The odd relative offset.
        offset: u32,
    },
    /// A `loca` entry points beyond the unpadded `glyf` table.
    LocaOffsetOutOfBounds {
        /// The zero-based location-entry index.
        index: u32,
        /// The relative offset outside `glyf`.
        offset: u32,
        /// The unpadded `glyf` table length.
        glyf_length: usize,
    },
    /// The final `loca` sentinel leaves trailing bytes in `glyf`.
    LocaEndMismatch {
        /// The zero-based sentinel-entry index.
        index: u32,
        /// The final relative offset.
        offset: u32,
        /// The unpadded `glyf` table length.
        glyf_length: usize,
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
            Self::NonzeroLocaStart { offset } => {
                write!(f, "loca first offset {offset} does not start glyf at zero")
            }
            Self::DecreasingLocaOffset {
                index,
                offset,
                previous,
            } => write!(
                f,
                "loca offset {index} value {offset} precedes previous offset {previous}"
            ),
            Self::MisalignedLocaOffset { index, offset } => {
                write!(f, "loca offset {index} value {offset} is not word aligned")
            }
            Self::LocaOffsetOutOfBounds {
                index,
                offset,
                glyf_length,
            } => write!(
                f,
                "loca offset {index} value {offset} exceeds glyf length {glyf_length}"
            ),
            Self::LocaEndMismatch {
                index,
                offset,
                glyf_length,
            } => write!(
                f,
                "loca sentinel {index} value {offset} does not equal glyf length {glyf_length}"
            ),
        }
    }
}

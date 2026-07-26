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
    /// Simple-glyph decoding was requested for a glyph without description bytes.
    EmptyGlyphDescription,
    /// A negative contour count identifies a composite rather than a simple glyph.
    CompositeGlyphForSimpleDecoder {
        /// The signed contour count read from the Glyph Header.
        number_of_contours: i16,
    },
    /// The Glyph Header records an inverted bounding axis.
    InvalidGlyphBounds {
        /// The recorded minimum horizontal coordinate.
        minimum_x: i16,
        /// The recorded minimum vertical coordinate.
        minimum_y: i16,
        /// The recorded maximum horizontal coordinate.
        maximum_x: i16,
        /// The recorded maximum vertical coordinate.
        maximum_y: i16,
    },
    /// A simple glyph contour does not end after its predecessor.
    NonIncreasingContourEnd {
        /// The zero-based contour index.
        contour_index: u16,
        /// The repeated or decreasing inclusive point index.
        end_point: u16,
        /// The preceding contour's inclusive point index.
        previous_end_point: u16,
    },
    /// A packed flag run expands past the simple glyph's point count.
    SimpleFlagRepeatOutOfBounds {
        /// The first logical point receiving the physical flag.
        point_index: u32,
        /// The number of additional repetitions encoded after the flag.
        additional_repetitions: u8,
        /// The exact logical point count derived from contour ends.
        point_count: u32,
    },
    /// A simple glyph flag sets the reserved high bit.
    ReservedSimpleGlyphFlag {
        /// The first logical point receiving the physical flag.
        point_index: u32,
        /// The complete offending flag byte.
        flag: u8,
    },
    /// `OVERLAP_SIMPLE` appears on a physical flag after the first.
    MisplacedSimpleOverlapFlag {
        /// The first logical point receiving the physical flag.
        point_index: u32,
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
                "{format} loca length {actual} does not equal {expected} bytes required by {glyph_count} glyph(s)"
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
            Self::EmptyGlyphDescription => {
                f.write_str("simple glyph decoder received an empty glyph description")
            }
            Self::CompositeGlyphForSimpleDecoder { number_of_contours } => write!(
                f,
                "simple glyph decoder received composite contour count {number_of_contours}"
            ),
            Self::InvalidGlyphBounds {
                minimum_x,
                minimum_y,
                maximum_x,
                maximum_y,
            } => write!(
                f,
                "glyph bounds ({minimum_x}, {minimum_y})..({maximum_x}, {maximum_y}) are inverted"
            ),
            Self::NonIncreasingContourEnd {
                contour_index,
                end_point,
                previous_end_point,
            } => write!(
                f,
                "simple contour {contour_index} ends at point {end_point}, not after point {previous_end_point}"
            ),
            Self::SimpleFlagRepeatOutOfBounds {
                point_index,
                additional_repetitions,
                point_count,
            } => write!(
                f,
                "simple flag at point {point_index} repeats {additional_repetitions} additional time(s) past {point_count} point(s)"
            ),
            Self::ReservedSimpleGlyphFlag { point_index, flag } => write!(
                f,
                "simple flag at point {point_index} sets reserved bits in {flag:#04x}"
            ),
            Self::MisplacedSimpleOverlapFlag { point_index } => write!(
                f,
                "simple overlap flag appears after the first physical flag at point {point_index}"
            ),
        }
    }
}

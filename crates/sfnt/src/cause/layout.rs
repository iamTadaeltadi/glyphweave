//! OpenType layout-specific diagnostic causes.

use core::fmt;

/// A format or invariant failure in a shared OpenType layout structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutCause {
    /// A Coverage table used an unknown format selector.
    UnsupportedCoverageFormat {
        /// The unrecognized selector.
        format: u16,
    },
    /// Coverage format 1 did not list glyph IDs in strict ascending order.
    NonIncreasingCoverageGlyph {
        /// The preceding glyph ID.
        previous: crate::GlyphId,
        /// The repeated or decreasing glyph ID.
        current: crate::GlyphId,
    },
    /// A Coverage range ended before it began.
    InvalidCoverageRange {
        /// The first glyph ID in the range.
        start: crate::GlyphId,
        /// The last glyph ID in the range.
        end: crate::GlyphId,
    },
    /// A Coverage range began at or before the preceding range ended.
    OverlappingCoverageRange {
        /// The preceding range's final glyph ID.
        previous_end: crate::GlyphId,
        /// The next range's first glyph ID.
        start: crate::GlyphId,
    },
    /// A format 2 range did not continue the sequential Coverage Index space.
    DiscontinuousCoverageIndex {
        /// The index derived from preceding ranges.
        expected: u32,
        /// The index stored in the range record.
        actual: u16,
    },
    /// A Class Definition table used an unknown format selector.
    UnsupportedClassDefinitionFormat {
        /// The unrecognized selector.
        format: u16,
    },
    /// A format 1 Class Definition range extends beyond glyph ID 65535.
    ClassDefinitionGlyphRangeOverflow {
        /// The first glyph ID in the dense range.
        start: crate::GlyphId,
        /// The number of class values in the range.
        count: u16,
    },
    /// A Class Definition range ended before it began.
    InvalidClassDefinitionRange {
        /// The first glyph ID in the range.
        start: crate::GlyphId,
        /// The last glyph ID in the range.
        end: crate::GlyphId,
    },
    /// A Class Definition range began at or before the preceding range ended.
    OverlappingClassDefinitionRange {
        /// The preceding range's final glyph ID.
        previous_end: crate::GlyphId,
        /// The next range's first glyph ID.
        start: crate::GlyphId,
    },
}

impl fmt::Display for LayoutCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnsupportedCoverageFormat { format } => {
                write!(f, "unsupported Coverage format {format}")
            }
            Self::NonIncreasingCoverageGlyph { previous, current } => write!(
                f,
                "Coverage glyph {current} does not strictly follow {previous}"
            ),
            Self::InvalidCoverageRange { start, end } => {
                write!(f, "Coverage range {start}..={end} has reversed bounds")
            }
            Self::OverlappingCoverageRange {
                previous_end,
                start,
            } => write!(
                f,
                "Coverage range beginning at {start} overlaps preceding end {previous_end}"
            ),
            Self::DiscontinuousCoverageIndex { expected, actual } => write!(
                f,
                "Coverage range index {actual} does not continue at {expected}"
            ),
            Self::UnsupportedClassDefinitionFormat { format } => {
                write!(f, "unsupported Class Definition format {format}")
            }
            Self::ClassDefinitionGlyphRangeOverflow { start, count } => write!(
                f,
                "Class Definition range beginning at {start} with {count} value(s) exceeds glyph ID 65535"
            ),
            Self::InvalidClassDefinitionRange { start, end } => write!(
                f,
                "Class Definition range {start}..={end} has reversed bounds"
            ),
            Self::OverlappingClassDefinitionRange {
                previous_end,
                start,
            } => write!(
                f,
                "Class Definition range beginning at {start} overlaps preceding end {previous_end}"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutCause;
    use crate::GlyphId;

    #[test]
    fn unsupported_coverage_format_names_the_selector() {
        let cause = LayoutCause::UnsupportedCoverageFormat { format: 9 };
        assert_eq!(cause.to_string(), "unsupported Coverage format 9");
    }

    #[test]
    fn unsupported_class_definition_format_names_the_selector() {
        let cause = LayoutCause::UnsupportedClassDefinitionFormat { format: 7 };
        assert_eq!(cause.to_string(), "unsupported Class Definition format 7");
    }

    #[test]
    fn coverage_ordering_cause_names_both_glyphs() {
        let cause = LayoutCause::NonIncreasingCoverageGlyph {
            previous: GlyphId::new(9),
            current: GlyphId::new(7),
        };
        assert_eq!(
            cause.to_string(),
            "Coverage glyph 7 does not strictly follow 9"
        );
    }

    #[test]
    fn invalid_coverage_range_names_its_bounds() {
        let cause = LayoutCause::InvalidCoverageRange {
            start: GlyphId::new(12),
            end: GlyphId::new(10),
        };
        assert_eq!(
            cause.to_string(),
            "Coverage range 12..=10 has reversed bounds"
        );
    }

    #[test]
    fn discontinuous_coverage_index_names_expected_and_actual() {
        let cause = LayoutCause::DiscontinuousCoverageIndex {
            expected: 20,
            actual: 18,
        };
        assert_eq!(
            cause.to_string(),
            "Coverage range index 18 does not continue at 20"
        );
    }

    #[test]
    fn overlapping_coverage_range_names_the_boundary() {
        let cause = LayoutCause::OverlappingCoverageRange {
            previous_end: GlyphId::new(15),
            start: GlyphId::new(15),
        };
        assert_eq!(
            cause.to_string(),
            "Coverage range beginning at 15 overlaps preceding end 15"
        );
    }

    #[test]
    fn class_range_overflow_names_start_and_count() {
        let cause = LayoutCause::ClassDefinitionGlyphRangeOverflow {
            start: GlyphId::new(65_534),
            count: 3,
        };
        assert_eq!(
            cause.to_string(),
            "Class Definition range beginning at 65534 with 3 value(s) exceeds glyph ID 65535"
        );
    }

    #[test]
    fn invalid_class_range_names_its_bounds() {
        let cause = LayoutCause::InvalidClassDefinitionRange {
            start: GlyphId::new(8),
            end: GlyphId::new(7),
        };
        assert_eq!(
            cause.to_string(),
            "Class Definition range 8..=7 has reversed bounds"
        );
    }

    #[test]
    fn overlapping_class_range_names_the_boundary() {
        let cause = LayoutCause::OverlappingClassDefinitionRange {
            previous_end: GlyphId::new(20),
            start: GlyphId::new(19),
        };
        assert_eq!(
            cause.to_string(),
            "Class Definition range beginning at 19 overlaps preceding end 20"
        );
    }
}

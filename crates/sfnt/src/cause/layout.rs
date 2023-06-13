//! OpenType layout-specific diagnostic causes.

use core::fmt;

/// The tag-ordered layout list involved in a validation failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutListKind {
    /// The ScriptRecord array of a ScriptList.
    Script,
    /// The LangSysRecord array of a Script table.
    Language,
    /// The FeatureRecord array of a FeatureList.
    Feature,
}

impl fmt::Display for LayoutListKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Script => "script",
            Self::Language => "language",
            Self::Feature => "feature",
        })
    }
}

/// The relative-offset target involved in a layout validation failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutTarget {
    /// A Script table referenced by ScriptList.
    Script,
    /// The default Language System table of a Script.
    DefaultLanguageSystem,
    /// A named Language System table referenced by a Script.
    LanguageSystem,
    /// A Feature table referenced by FeatureList.
    Feature,
    /// A tag-specific Feature Parameters table.
    FeatureParameters,
}

impl fmt::Display for LayoutTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Script => "Script",
            Self::DefaultLanguageSystem => "default Language System",
            Self::LanguageSystem => "Language System",
            Self::Feature => "Feature",
            Self::FeatureParameters => "Feature Parameters",
        })
    }
}

/// A format or invariant failure in a shared OpenType layout structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutCause {
    /// A Script, Language System, or Feature record used a malformed Tag.
    InvalidLayoutTag {
        /// The list containing the record.
        list: LayoutListKind,
        /// The malformed Tag.
        tag: crate::Tag,
    },
    /// A tagged record array decreased instead of remaining alphabetically ordered.
    UnsortedLayoutTags {
        /// The list containing the records.
        list: LayoutListKind,
        /// The preceding Tag.
        previous: crate::Tag,
        /// The decreasing Tag.
        current: crate::Tag,
    },
    /// A uniquely keyed tagged record array repeated a Tag.
    DuplicateLayoutTag {
        /// The list containing the records.
        list: LayoutListKind,
        /// The repeated Tag.
        tag: crate::Tag,
    },
    /// An Offset16 target was null, inside its owner records, or outside its container.
    InvalidLayoutOffset {
        /// The kind of structure being targeted.
        target: LayoutTarget,
        /// The stored relative Offset16.
        offset: u16,
        /// The first legal relative target byte.
        minimum: usize,
        /// The containing structure's byte length.
        container_length: usize,
    },
    /// A Language System table used the reserved lookup-order offset.
    ReservedLookupOrderOffset {
        /// The nonzero reserved value.
        offset: u16,
    },
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
            Self::InvalidLayoutTag { list, tag } => {
                write!(f, "invalid {list} tag {tag:?}")
            }
            Self::UnsortedLayoutTags {
                list,
                previous,
                current,
            } => write!(f, "{list} tag {current} follows larger tag {previous}"),
            Self::DuplicateLayoutTag { list, tag } => {
                write!(f, "duplicate {list} tag {tag}")
            }
            Self::InvalidLayoutOffset {
                target,
                offset,
                minimum,
                container_length,
            } => write!(
                f,
                "{target} offset {offset} is outside legal range {minimum}..{container_length}"
            ),
            Self::ReservedLookupOrderOffset { offset } => {
                write!(f, "reserved lookup-order offset must be zero, not {offset}")
            }
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
mod tests;

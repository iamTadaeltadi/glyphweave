//! Domain-specific causes carried by SFNT Diagnostics.

use core::fmt;

/// The domain-specific cause of a Diagnostic.
///
/// Causes grow as later stages are implemented; the enum is `#[non_exhaustive]` so that a new
/// variant never breaks a caller that already matches it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Cause {
    /// The Table Directory declared a version outside the OpenType enumeration.
    UnsupportedSfntVersion {
        /// The unrecognized four-byte value.
        raw: u32,
    },
    /// The record count cannot be represented by the directory search fields.
    UnrepresentableTableCount {
        /// The count declared by the directory.
        count: u16,
        /// The greatest count encodable by all search fields.
        maximum: u16,
    },
    /// Header search fields disagree with values derived from the table count.
    InvalidSearchParameters {
        /// The values `(searchRange, entrySelector, rangeShift)` required by the count.
        expected: [u16; 3],
        /// The values stored in the font.
        actual: [u16; 3],
    },
    /// A Table Record points outside the containing font bytes.
    TableOutOfBounds {
        /// The offending table.
        tag: crate::Tag,
        /// The declared offset from the start of the font file.
        offset: u32,
        /// The declared unpadded table length.
        length: u32,
        /// The available font length.
        font_length: usize,
    },
    /// A Table Record tag violates the OpenType Tag byte grammar.
    InvalidTableTag {
        /// The four offending bytes.
        tag: crate::Tag,
    },
    /// Two records declare the same table Tag.
    DuplicateTableTag {
        /// The repeated Tag.
        tag: crate::Tag,
    },
    /// Table Records are not in strict ascending Tag order.
    UnsortedTableTags {
        /// The Tag immediately before the ordering violation.
        previous: crate::Tag,
        /// The Tag that should have sorted before `previous`.
        current: crate::Tag,
    },
    /// A top-level table does not begin on a four-byte boundary.
    MisalignedTable {
        /// The offending table.
        tag: crate::Tag,
        /// Its declared absolute offset.
        offset: u32,
    },
    /// A bounded read needed more bytes than remained in the input.
    UnexpectedEnd {
        /// The number of bytes the read required.
        needed: usize,
        /// The number of bytes that remained.
        available: usize,
    },
    /// A Byte Span ended past the bytes it was observed against.
    SpanOutOfBounds {
        /// The exclusive end of the offending span.
        end: usize,
        /// The length of the bytes the span was observed against.
        length: usize,
    },
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnsupportedSfntVersion { raw } => {
                write!(f, "unsupported SFNT version 0x{raw:08x}")
            }
            Self::UnrepresentableTableCount { count, maximum } => {
                write!(
                    f,
                    "table count {count} exceeds representable maximum {maximum}"
                )
            }
            Self::InvalidSearchParameters { expected, actual } => write!(
                f,
                "directory search parameters {actual:?} do not match derived values {expected:?}"
            ),
            Self::TableOutOfBounds {
                tag,
                offset,
                length,
                font_length,
            } => write!(
                f,
                "table {tag} at {offset}+{length} exceeds font length {font_length}"
            ),
            Self::InvalidTableTag { tag } => write!(f, "invalid table tag {tag:?}"),
            Self::DuplicateTableTag { tag } => write!(f, "duplicate table tag {tag}"),
            Self::UnsortedTableTags { previous, current } => {
                write!(f, "table tag {current} follows larger tag {previous}")
            }
            Self::MisalignedTable { tag, offset } => {
                write!(f, "table {tag} starts at unaligned offset {offset}")
            }
            Self::UnexpectedEnd { needed, available } => {
                write!(f, "needed {needed} byte(s) but only {available} remained")
            }
            Self::SpanOutOfBounds { end, length } => {
                write!(
                    f,
                    "span ends at {end} but only {length} byte(s) are available"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cause;

    #[test]
    fn unsupported_version_preserves_all_four_bytes() {
        let cause = Cause::UnsupportedSfntVersion { raw: 0x7472_7565 };
        assert_eq!(cause.to_string(), "unsupported SFNT version 0x74727565");
    }

    #[test]
    fn table_count_failure_names_the_limit() {
        let cause = Cause::UnrepresentableTableCount {
            count: 4_096,
            maximum: 4_095,
        };
        assert_eq!(
            cause.to_string(),
            "table count 4096 exceeds representable maximum 4095"
        );
    }

    #[test]
    fn search_parameter_failure_shows_both_triplets() {
        let cause = Cause::InvalidSearchParameters {
            expected: [128, 3, 80],
            actual: [64, 2, 144],
        };
        assert_eq!(
            cause.to_string(),
            "directory search parameters [64, 2, 144] do not match derived values [128, 3, 80]"
        );
    }

    #[test]
    fn table_bounds_failure_names_the_record() {
        let cause = Cause::TableOutOfBounds {
            tag: crate::Tag::new(*b"glyf"),
            offset: 64,
            length: 40,
            font_length: 96,
        };
        assert_eq!(
            cause.to_string(),
            "table glyf at 64+40 exceeds font length 96"
        );
    }

    #[test]
    fn invalid_tag_failure_uses_unambiguous_debug_form() {
        let cause = Cause::InvalidTableTag {
            tag: crate::Tag::new(*b"a b "),
        };
        assert_eq!(cause.to_string(), "invalid table tag Tag(a b )");
    }

    #[test]
    fn duplicate_tag_failure_names_the_tag() {
        let cause = Cause::DuplicateTableTag {
            tag: crate::Tag::new(*b"cmap"),
        };
        assert_eq!(cause.to_string(), "duplicate table tag cmap");
    }

    #[test]
    fn unsorted_tag_failure_names_the_adjacent_pair() {
        let cause = Cause::UnsortedTableTags {
            previous: crate::Tag::new(*b"head"),
            current: crate::Tag::new(*b"cmap"),
        };
        assert_eq!(cause.to_string(), "table tag cmap follows larger tag head");
    }

    #[test]
    fn alignment_failure_names_table_and_offset() {
        let cause = Cause::MisalignedTable {
            tag: crate::Tag::new(*b"head"),
            offset: 29,
        };
        assert_eq!(
            cause.to_string(),
            "table head starts at unaligned offset 29"
        );
    }
}

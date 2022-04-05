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
}

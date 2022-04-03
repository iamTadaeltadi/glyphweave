//! Domain-specific causes carried by SFNT Diagnostics.

use core::fmt;

/// The domain-specific cause of a Diagnostic.
///
/// Causes grow as later stages are implemented; the enum is `#[non_exhaustive]` so that a new
/// variant never breaks a caller that already matches it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Cause {
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

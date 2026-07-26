//! The Byte Span primitive: a checked half-open range into immutable bytes.

use core::fmt;

use crate::cause::Cause;
use crate::diagnostic::{Diagnostic, Stage};

/// A checked half-open byte range `[start, end)` into an immutable input or output artifact.
///
/// A `ByteSpan` records *where* a value lives in a byte sequence. It never borrows the bytes,
/// so it can be stored inside decoded values and Diagnostics without tying them to an input
/// lifetime. The type maintains one invariant: `start <= end`, so a span always has a
/// non-negative length. Observing bytes through a span is fallible and bounds-checked by
/// [`ByteSpan::slice`], which upholds the pipeline rule that a span is proven in bounds before
/// any byte is read.
///
/// ```
/// use glyphweave_sfnt::ByteSpan;
///
/// let span = ByteSpan::sized(1, 2).expect("no overflow");
/// assert_eq!(span.slice(&[9, 8, 7, 6]).unwrap(), &[8, 7]);
/// assert!(span.slice(&[9, 8]).is_err());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteSpan {
    start: usize,
    end: usize,
}

impl ByteSpan {
    /// Creates the span `[start, start + length)`.
    ///
    /// Returns `None` when `start + length` overflows `usize`, keeping span arithmetic total
    /// on every target width.
    #[must_use]
    pub const fn sized(start: usize, length: usize) -> Option<Self> {
        match start.checked_add(length) {
            Some(end) => Some(Self { start, end }),
            None => None,
        }
    }

    /// Creates the span `[start, end)`.
    ///
    /// Returns `None` when `end < start`, so an inverted span can never be constructed.
    #[must_use]
    pub const fn between(start: usize, end: usize) -> Option<Self> {
        if end < start {
            None
        } else {
            Some(Self { start, end })
        }
    }

    /// Creates the empty span `[offset, offset)`.
    #[must_use]
    pub const fn empty_at(offset: usize) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    /// The inclusive start offset.
    #[must_use]
    pub const fn start(self) -> usize {
        self.start
    }

    /// The exclusive end offset.
    #[must_use]
    pub const fn end(self) -> usize {
        self.end
    }

    /// The number of bytes the span covers.
    #[must_use]
    pub const fn length(self) -> usize {
        self.end - self.start
    }

    /// Returns `true` when the span covers no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns `true` when `offset` lies within `[start, end)`.
    #[must_use]
    pub const fn contains(self, offset: usize) -> bool {
        self.start <= offset && offset < self.end
    }

    /// Returns the smallest span covering both `self` and `other`.
    #[must_use]
    pub const fn join(self, other: Self) -> Self {
        let start = if self.start < other.start {
            self.start
        } else {
            other.start
        };
        let end = if self.end > other.end {
            self.end
        } else {
            other.end
        };
        Self { start, end }
    }

    /// Returns `true` when both spans cover at least one common byte.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        !self.is_empty() && !other.is_empty() && self.start < other.end && other.start < self.end
    }

    /// Borrows the bytes the span covers.
    ///
    /// # Errors
    ///
    /// Returns a [`Stage::Decode`] Diagnostic carrying [`Cause::SpanOutOfBounds`] when the span
    /// ends past `bytes`, upholding the invariant that a span is in bounds before observation.
    pub fn slice(self, bytes: &[u8]) -> Result<&[u8], Diagnostic> {
        if self.end <= bytes.len() {
            Ok(&bytes[self.start..self.end])
        } else {
            Err(Diagnostic::at(
                Stage::Decode,
                self,
                Cause::SpanOutOfBounds {
                    end: self.end,
                    length: bytes.len(),
                },
            ))
        }
    }
}

impl fmt::Debug for ByteSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ByteSpan({}..{})", self.start, self.end)
    }
}

impl fmt::Display for ByteSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::ByteSpan;
    use crate::cause::Cause;

    #[test]
    fn sized_rejects_overflow() {
        assert!(ByteSpan::sized(usize::MAX, 1).is_none());
        let span = ByteSpan::sized(4, 3).unwrap();
        assert_eq!((span.start(), span.end(), span.length()), (4, 7, 3));
    }

    #[test]
    fn between_rejects_inverted_ranges() {
        assert!(ByteSpan::between(7, 4).is_none());
        let span = ByteSpan::between(2, 5).unwrap();
        assert_eq!(span.length(), 3);
    }

    #[test]
    fn empty_span_covers_nothing() {
        let span = ByteSpan::empty_at(9);
        assert!(span.is_empty());
        assert_eq!(span.length(), 0);
        assert_eq!(span.slice(&[0; 12]).unwrap(), &[] as &[u8]);
    }

    #[test]
    fn contains_uses_half_open_bounds() {
        let span = ByteSpan::between(2, 5).unwrap();
        assert!(!span.contains(1));
        assert!(span.contains(2));
        assert!(span.contains(4));
        assert!(!span.contains(5));
    }

    #[test]
    fn join_covers_both_spans() {
        let left = ByteSpan::between(2, 4).unwrap();
        let right = ByteSpan::between(6, 9).unwrap();
        let joined = left.join(right);
        assert_eq!((joined.start(), joined.end()), (2, 9));
        assert_eq!(joined, right.join(left));
    }

    #[test]
    fn overlap_excludes_touching_and_empty_spans() {
        let left = ByteSpan::between(2, 6).unwrap();
        assert!(left.overlaps(ByteSpan::between(5, 8).unwrap()));
        assert!(!left.overlaps(ByteSpan::between(6, 9).unwrap()));
        assert!(!left.overlaps(ByteSpan::empty_at(4)));
        assert!(!ByteSpan::empty_at(4).overlaps(left));
    }

    #[test]
    fn slice_borrows_covered_bytes() {
        let span = ByteSpan::sized(1, 2).unwrap();
        assert_eq!(span.slice(&[10, 20, 30, 40]).unwrap(), &[20, 30]);
    }

    #[test]
    fn slice_reports_out_of_bounds() {
        let span = ByteSpan::between(2, 6).unwrap();
        let diagnostic = span.slice(&[0, 1, 2, 3]).unwrap_err();
        assert_eq!(diagnostic.span(), Some(span));
        assert_eq!(
            diagnostic.cause(),
            Cause::SpanOutOfBounds { end: 6, length: 4 }
        );
    }

    #[test]
    fn debug_and_display_render_the_range() {
        let span = ByteSpan::between(3, 8).unwrap();
        assert_eq!(format!("{span}"), "3..8");
        assert_eq!(format!("{span:?}"), "ByteSpan(3..8)");
    }
}

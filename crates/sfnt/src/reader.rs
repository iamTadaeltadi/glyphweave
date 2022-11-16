//! Bounded big-endian reads over immutable bytes.

use crate::cause::Cause;
use crate::diagnostic::{Diagnostic, Stage};
use crate::span::ByteSpan;
use crate::tag::Tag;

/// A forward cursor that reads big-endian scalars from immutable bytes without unchecked access.
///
/// Every read advances the cursor and yields either a value or a [`Stage::Decode`] Diagnostic
/// that carries the [`ByteSpan`] it attempted to read. The reader never copies the underlying
/// bytes and never reads outside them, making it the single choke point that upholds the
/// crate's bounded-decoding guarantee. The cursor offset is always in `0..=len`.
///
/// ```
/// use glyphweave_sfnt::Reader;
///
/// let mut reader = Reader::new(&[0x00, 0x10, b'g', b'l', b'y', b'f']);
/// assert_eq!(reader.u16().unwrap(), 0x0010);
/// assert_eq!(reader.tag().unwrap().to_string(), "glyf");
/// assert!(reader.u8().is_err());
/// ```
#[derive(Clone, Debug)]
pub struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
    end: usize,
}

impl<'a> Reader<'a> {
    /// Creates a reader positioned at the start of `bytes`.
    #[must_use]
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            offset: 0,
            end: bytes.len(),
        }
    }

    /// Creates a reader positioned at `offset` within `bytes`.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::SpanOutOfBounds`] Diagnostic when `offset` is past the end of `bytes`.
    pub fn at(bytes: &'a [u8], offset: usize) -> Result<Self, Diagnostic> {
        if offset <= bytes.len() {
            Ok(Self {
                bytes,
                offset,
                end: bytes.len(),
            })
        } else {
            let span = ByteSpan::between(bytes.len(), offset)
                .unwrap_or_else(|| ByteSpan::empty_at(bytes.len()));
            Err(Diagnostic::at(
                Stage::Decode,
                span,
                Cause::SpanOutOfBounds {
                    end: offset,
                    length: bytes.len(),
                },
            ))
        }
    }

    /// Creates a reader confined to one absolute checked span in `bytes`.
    ///
    /// Reads cannot cross the span's end even when more input bytes follow it. Diagnostics retain
    /// absolute offsets into `bytes`.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::SpanOutOfBounds`] Diagnostic when `span` exceeds `bytes`.
    pub fn within(bytes: &'a [u8], span: ByteSpan) -> Result<Self, Diagnostic> {
        span.slice(bytes)?;
        Ok(Self {
            bytes,
            offset: span.start(),
            end: span.end(),
        })
    }

    /// The current cursor offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// The number of bytes remaining after the cursor.
    #[must_use]
    pub const fn remaining(&self) -> usize {
        self.end - self.offset
    }

    /// Returns `true` when no bytes remain after the cursor.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.offset >= self.end
    }

    /// Reads one unsigned byte.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when no byte remains.
    pub fn u8(&mut self) -> Result<u8, Diagnostic> {
        Ok(self.take::<1>()?[0])
    }

    /// Reads a big-endian unsigned 16-bit integer.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than two bytes remain.
    pub fn u16(&mut self) -> Result<u16, Diagnostic> {
        Ok(u16::from_be_bytes(self.take::<2>()?))
    }

    /// Reads a big-endian signed 16-bit integer.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than two bytes remain.
    pub fn i16(&mut self) -> Result<i16, Diagnostic> {
        Ok(i16::from_be_bytes(self.take::<2>()?))
    }

    /// Reads a big-endian unsigned 24-bit integer into a `u32`.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than three bytes remain.
    pub fn u24(&mut self) -> Result<u32, Diagnostic> {
        let [high, middle, low] = self.take::<3>()?;
        Ok(u32::from_be_bytes([0, high, middle, low]))
    }

    /// Reads a big-endian unsigned 32-bit integer.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than four bytes remain.
    pub fn u32(&mut self) -> Result<u32, Diagnostic> {
        Ok(u32::from_be_bytes(self.take::<4>()?))
    }

    /// Reads a four-byte [`Tag`].
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than four bytes remain.
    pub fn tag(&mut self) -> Result<Tag, Diagnostic> {
        Ok(Tag::new(self.take::<4>()?))
    }

    /// Borrows the next `length` bytes without copying and advances past them.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than `length` bytes remain.
    pub fn array(&mut self, length: usize) -> Result<&'a [u8], Diagnostic> {
        match self.offset.checked_add(length) {
            Some(end) if end <= self.end => {
                let slice = &self.bytes[self.offset..end];
                self.offset = end;
                Ok(slice)
            }
            _ => Err(self.unexpected_end(length)),
        }
    }

    /// Records the [`ByteSpan`] of the next `length` bytes and advances past them without reading.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than `length` bytes remain.
    pub fn take_span(&mut self, length: usize) -> Result<ByteSpan, Diagnostic> {
        let start = self.offset;
        self.array(length)?;
        // `array` succeeded, so `start + length` neither overflowed nor left the buffer.
        Ok(ByteSpan::sized(start, length).unwrap_or_else(|| ByteSpan::empty_at(start)))
    }

    /// Advances the cursor past `length` bytes.
    ///
    /// # Errors
    ///
    /// Returns a [`Cause::UnexpectedEnd`] Diagnostic when fewer than `length` bytes remain.
    pub fn skip(&mut self, length: usize) -> Result<(), Diagnostic> {
        self.array(length).map(|_| ())
    }

    /// Reads `N` bytes into a fixed array, advancing the cursor only on success.
    fn take<const N: usize>(&mut self) -> Result<[u8; N], Diagnostic> {
        match self.offset.checked_add(N) {
            Some(end) if end <= self.end => {
                let mut buffer = [0u8; N];
                buffer.copy_from_slice(&self.bytes[self.offset..end]);
                self.offset = end;
                Ok(buffer)
            }
            _ => Err(self.unexpected_end(N)),
        }
    }

    /// Builds the truncation Diagnostic for a read of `needed` bytes at the current offset.
    fn unexpected_end(&self, needed: usize) -> Diagnostic {
        let available = self.remaining();
        let span = ByteSpan::between(self.offset, self.end)
            .unwrap_or_else(|| ByteSpan::empty_at(self.offset));
        Diagnostic::at(
            Stage::Decode,
            span,
            Cause::UnexpectedEnd { needed, available },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Reader;
    use crate::cause::Cause;

    #[test]
    fn new_positions_at_the_start() {
        let reader = Reader::new(&[1, 2, 3]);
        assert_eq!(reader.offset(), 0);
        assert_eq!(reader.remaining(), 3);
        assert!(!reader.is_empty());
    }

    #[test]
    fn at_accepts_offsets_up_to_the_length() {
        let bytes = [1, 2, 3, 4];
        let reader = Reader::at(&bytes, 4).unwrap();
        assert!(reader.is_empty());
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn at_rejects_offsets_past_the_end() {
        let diagnostic = Reader::at(&[1, 2], 3).unwrap_err();
        assert_eq!(
            diagnostic.cause(),
            Cause::SpanOutOfBounds { end: 3, length: 2 }
        );
        assert_eq!(diagnostic.span().unwrap().start(), 2);
    }

    #[test]
    fn within_starts_and_ends_at_absolute_span_boundaries() {
        let bytes = [0xaa, 0xbb, 0xcc, 0xdd, 0xee];
        let span = crate::ByteSpan::between(1, 4).unwrap();
        let mut reader = Reader::within(&bytes, span).unwrap();

        assert_eq!(reader.offset(), 1);
        assert_eq!(reader.remaining(), 3);
        assert_eq!(reader.u16().unwrap(), 0xbbcc);
        let diagnostic = reader.u16().unwrap_err();
        assert_eq!(
            diagnostic.cause(),
            Cause::UnexpectedEnd {
                needed: 2,
                available: 1
            }
        );
        assert_eq!(diagnostic.span(), crate::ByteSpan::between(3, 4));
    }

    #[test]
    fn within_rejects_a_span_past_the_input() {
        let span = crate::ByteSpan::between(1, 4).unwrap();
        let diagnostic = Reader::within(&[0, 1, 2], span).unwrap_err();

        assert_eq!(
            diagnostic.cause(),
            Cause::SpanOutOfBounds { end: 4, length: 3 }
        );
        assert_eq!(diagnostic.span(), Some(span));
    }

    #[test]
    fn reads_big_endian_scalars_in_order() {
        let bytes = [
            0x12, 0x34, 0x56, 0xff, 0xfe, 0xab, 0xcd, 0xef, 0x00, 0x00, 0x01, 0x00,
        ];
        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.u8().unwrap(), 0x12);
        assert_eq!(reader.u16().unwrap(), 0x3456);
        assert_eq!(reader.i16().unwrap(), -2);
        assert_eq!(reader.u24().unwrap(), 0x00ab_cdef);
        assert_eq!(reader.u32().unwrap(), 0x0000_0100);
        assert_eq!(reader.offset(), 12);
    }

    #[test]
    fn reads_a_tag() {
        let mut reader = Reader::new(b"GSUBrest");
        assert_eq!(reader.tag().unwrap().to_string(), "GSUB");
        assert_eq!(reader.remaining(), 4);
    }

    #[test]
    fn array_borrows_without_copying() {
        let bytes = [9, 8, 7, 6, 5];
        let mut reader = Reader::new(&bytes);
        assert_eq!(reader.array(2).unwrap(), &[9, 8]);
        assert_eq!(reader.array(3).unwrap(), &[7, 6, 5]);
        assert!(reader.is_empty());
    }

    #[test]
    fn take_span_records_the_read_region() {
        let mut reader = Reader::new(&[0; 10]);
        reader.skip(3).unwrap();
        let span = reader.take_span(4).unwrap();
        assert_eq!((span.start(), span.end()), (3, 7));
        assert_eq!(reader.offset(), 7);
    }

    #[test]
    fn truncated_read_reports_unexpected_end_with_a_span() {
        let mut reader = Reader::new(&[0xaa, 0xbb]);
        reader.u8().unwrap();
        let diagnostic = reader.u32().unwrap_err();
        assert_eq!(
            diagnostic.cause(),
            Cause::UnexpectedEnd {
                needed: 4,
                available: 1
            }
        );
        let span = diagnostic.span().unwrap();
        assert_eq!((span.start(), span.end()), (1, 2));
        // A failed read does not advance the cursor.
        assert_eq!(reader.offset(), 1);
    }

    #[test]
    fn array_truncation_reports_the_requested_length() {
        let mut reader = Reader::new(&[1, 2, 3]);
        let diagnostic = reader.array(5).unwrap_err();
        assert_eq!(
            diagnostic.cause(),
            Cause::UnexpectedEnd {
                needed: 5,
                available: 3
            }
        );
    }
}

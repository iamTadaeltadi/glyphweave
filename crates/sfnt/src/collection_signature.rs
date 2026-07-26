//! Location metadata for a file-level collection signature.

use crate::{ByteSpan, Decoded};

/// The checked range of a version 2.0 Font Collection's DSIG table.
///
/// This value proves only that the TTC descriptor is structurally valid. It does not assert that
/// the cryptographic signature is authentic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CollectionSignature {
    pub(crate) span: ByteSpan,
}

impl CollectionSignature {
    /// The absolute range containing the complete DSIG table.
    #[must_use]
    pub const fn span(self) -> ByteSpan {
        self.span
    }

    /// Borrows the signature table from its containing collection bytes.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when `bytes` is shorter than the collection this value was
    /// validated against.
    pub fn bytes(self, bytes: &[u8]) -> Decoded<&[u8]> {
        self.span.slice(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::CollectionSignature;
    use crate::ByteSpan;

    #[test]
    fn signature_borrows_its_validated_range() {
        let signature = CollectionSignature {
            span: ByteSpan::sized(4, 4).unwrap(),
        };
        assert_eq!(signature.span(), ByteSpan::sized(4, 4).unwrap());
        assert_eq!(signature.bytes(b"headDSIGtail").unwrap(), b"DSIG");
    }
}

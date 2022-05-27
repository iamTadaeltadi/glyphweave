//! A fully validated OpenType Font Collection.

use crate::cause::Cause;
use crate::{
    ByteSpan, CollectionSignature, CollectionVersion, Decoded, Diagnostic, Face, FaceIndex, Stage,
};

mod decode;

/// A decoded `ttcf` container whose advertised Faces are all validated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontCollection {
    pub(crate) version: CollectionVersion,
    pub(crate) header_span: ByteSpan,
    pub(crate) faces: Vec<Face>,
    pub(crate) signature: Option<CollectionSignature>,
}

impl FontCollection {
    /// Decodes and validates an OpenType Font Collection from complete file bytes.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when the TTC Header, any Face directory, shared table
    /// layout, or optional collection signature descriptor is malformed.
    pub fn decode(bytes: &[u8]) -> crate::Decoded<Self> {
        decode::decode(bytes)
    }

    /// The TTC Header version.
    #[must_use]
    pub const fn version(&self) -> CollectionVersion {
        self.version
    }

    /// The checked range occupied by the TTC Header and all directory offsets.
    #[must_use]
    pub const fn header_span(&self) -> ByteSpan {
        self.header_span
    }

    /// Every validated Face in Face Index order.
    #[must_use]
    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    /// Finds a Face by its zero-based Face Index.
    #[must_use]
    pub fn face(&self, index: FaceIndex) -> Option<&Face> {
        usize::try_from(index.raw())
            .ok()
            .and_then(|index| self.faces.get(index))
    }

    /// Selects a Face by its zero-based Face Index.
    ///
    /// # Errors
    ///
    /// Returns a Diagnostic when the requested index is not present.
    pub fn select(&self, index: FaceIndex) -> Decoded<&Face> {
        self.face(index).ok_or_else(|| {
            Diagnostic::failure(
                Stage::Decode,
                Cause::FaceIndexOutOfBounds {
                    index,
                    face_count: self.faces.len(),
                },
            )
        })
    }

    /// The number of Faces in this collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.faces.len()
    }

    /// Returns `true` when no Faces are present.
    ///
    /// Valid decoded collections are never empty; this method mirrors ordinary collection APIs.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    /// The optional file-level DSIG descriptor from a version 2.0 header.
    #[must_use]
    pub const fn signature(&self) -> Option<CollectionSignature> {
        self.signature
    }
}

#[cfg(test)]
mod tests {
    use super::FontCollection;
    use crate::{
        ByteSpan, Cause, CollectionSignature, CollectionVersion, Face, FaceIndex, SfntVersion,
        TableDirectory,
    };

    #[test]
    fn collection_preserves_face_index_order() {
        let face = Face {
            index: FaceIndex::FIRST,
            directory_offset: 28,
            directory: TableDirectory {
                version: SfntVersion::TrueType,
                span: ByteSpan::sized(28, 12).unwrap(),
                records: Vec::new(),
            },
        };
        let signature = CollectionSignature {
            span: ByteSpan::sized(40, 4).unwrap(),
        };
        let collection = FontCollection {
            version: CollectionVersion::Version2,
            header_span: ByteSpan::sized(0, 28).unwrap(),
            faces: vec![face.clone()],
            signature: Some(signature),
        };
        assert_eq!(collection.version(), CollectionVersion::Version2);
        assert_eq!(collection.header_span(), ByteSpan::sized(0, 28).unwrap());
        assert_eq!(collection.faces(), &[face]);
        assert_eq!(
            collection.face(FaceIndex::FIRST),
            collection.faces().first()
        );
        assert_eq!(collection.face(FaceIndex::new(1)), None);
        let missing = collection.select(FaceIndex::new(1)).unwrap_err();
        assert_eq!(
            missing.cause(),
            Cause::FaceIndexOutOfBounds {
                index: FaceIndex::new(1),
                face_count: 1
            }
        );
        assert_eq!(collection.len(), 1);
        assert!(!collection.is_empty());
        assert_eq!(collection.signature(), Some(signature));
    }
}

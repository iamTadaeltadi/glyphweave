//! One validated face selected from standalone or collection font bytes.

use crate::{FaceIndex, TableDirectory, TableRecord, Tag};

/// One decoded OpenType font resource.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Face {
    pub(crate) index: FaceIndex,
    pub(crate) directory_offset: usize,
    pub(crate) directory: TableDirectory,
}

impl Face {
    /// This Face's zero-based position in its containing font file.
    #[must_use]
    pub const fn index(&self) -> FaceIndex {
        self.index
    }

    /// The absolute byte offset where this Face's Table Directory begins.
    #[must_use]
    pub const fn directory_offset(&self) -> usize {
        self.directory_offset
    }

    /// The Face's decoded and validated Table Directory.
    #[must_use]
    pub const fn directory(&self) -> &TableDirectory {
        &self.directory
    }

    /// Finds a validated Table Record by Tag.
    #[must_use]
    pub fn record(&self, tag: Tag) -> Option<&TableRecord> {
        self.directory.record(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::Face;
    use crate::{ByteSpan, FaceIndex, SfntVersion, TableDirectory, Tag};

    #[test]
    fn face_exposes_its_identity_and_directory() {
        let directory = TableDirectory {
            version: SfntVersion::TrueType,
            span: ByteSpan::sized(24, 12).unwrap(),
            records: Vec::new(),
        };
        let face = Face {
            index: FaceIndex::new(2),
            directory_offset: 24,
            directory,
        };
        assert_eq!(face.index(), FaceIndex::new(2));
        assert_eq!(face.directory_offset(), 24);
        assert_eq!(face.directory().span(), ByteSpan::sized(24, 12).unwrap());
        assert_eq!(face.record(Tag::new(*b"name")), None);
    }
}

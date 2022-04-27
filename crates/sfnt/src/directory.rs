//! Typed SFNT Table Directory and Table Record values.

use crate::{ByteSpan, Checksum, SfntVersion, Tag};

mod decode;

/// A validated record locating one top-level table in an SFNT font.
///
/// ```
/// use glyphweave_sfnt::{TableDirectory, Tag};
///
/// let bytes = [
///     0, 1, 0, 0, 0, 1, 0, 16, 0, 0, 0, 0,
///     b'n', b'a', b'm', b'e', 0, 0, 0, 0,
///     0, 0, 0, 28, 0, 0, 0, 0,
/// ];
/// let directory = TableDirectory::decode(&bytes, 0).unwrap();
/// let record = directory.record(Tag::new(*b"name")).unwrap();
/// assert_eq!(record.table_span().to_string(), "28..28");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableRecord {
    pub(crate) tag: Tag,
    pub(crate) checksum: Checksum,
    pub(crate) table_span: ByteSpan,
    pub(crate) directory_span: ByteSpan,
}

impl TableRecord {
    /// The table's four-byte identifier.
    #[must_use]
    pub const fn tag(self) -> Tag {
        self.tag
    }

    /// The checksum declared by the Table Directory.
    #[must_use]
    pub const fn checksum(self) -> Checksum {
        self.checksum
    }

    /// The checked range containing the table's unpadded bytes.
    #[must_use]
    pub const fn table_span(self) -> ByteSpan {
        self.table_span
    }

    /// The range occupied by this record inside the Table Directory.
    #[must_use]
    pub const fn directory_span(self) -> ByteSpan {
        self.directory_span
    }
}

/// A decoded and structurally validated SFNT Table Directory.
///
/// ```
/// use glyphweave_sfnt::{SfntVersion, TableDirectory};
///
/// let bytes = [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
/// let directory = TableDirectory::decode(&bytes, 0).unwrap();
/// assert_eq!(directory.version(), SfntVersion::TrueType);
/// assert!(directory.is_empty());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableDirectory {
    pub(crate) version: SfntVersion,
    pub(crate) span: ByteSpan,
    pub(crate) records: Vec<TableRecord>,
}

impl TableDirectory {
    /// Decodes and validates a Table Directory at an absolute font-file offset.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, unsupported versions, invalid search
    /// parameters, or a Table Record that points outside `bytes`.
    pub fn decode(bytes: &[u8], offset: usize) -> crate::Decoded<Self> {
        decode::decode(bytes, offset)
    }

    /// The outline flavor declared by the directory.
    #[must_use]
    pub const fn version(&self) -> SfntVersion {
        self.version
    }

    /// The checked range occupied by the header and its records.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        self.span
    }

    /// The records in strict ascending Tag order.
    #[must_use]
    pub fn records(&self) -> &[TableRecord] {
        &self.records
    }

    /// Finds a table by Tag using the directory's validated ordering.
    #[must_use]
    pub fn record(&self, tag: Tag) -> Option<&TableRecord> {
        self.records
            .binary_search_by_key(&tag, |record| record.tag)
            .ok()
            .map(|index| &self.records[index])
    }

    /// Returns `true` when the directory has no table records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// The number of table records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{TableDirectory, TableRecord};
    use crate::{ByteSpan, Checksum, SfntVersion, Tag};

    #[test]
    fn directory_accessors_preserve_validated_records() {
        let record = TableRecord {
            tag: Tag::new(*b"cmap"),
            checksum: Checksum::new(7),
            table_span: ByteSpan::sized(32, 4).unwrap(),
            directory_span: ByteSpan::sized(12, 16).unwrap(),
        };
        let directory = TableDirectory {
            version: SfntVersion::TrueType,
            span: ByteSpan::sized(0, 28).unwrap(),
            records: vec![record],
        };
        assert_eq!(directory.version(), SfntVersion::TrueType);
        assert_eq!(directory.span().length(), 28);
        assert_eq!(directory.records(), &[record]);
        assert_eq!(directory.record(Tag::new(*b"cmap")), Some(&record));
        assert_eq!(directory.record(Tag::new(*b"head")), None);
        assert_eq!(directory.len(), 1);
        assert!(!directory.is_empty());
    }
}

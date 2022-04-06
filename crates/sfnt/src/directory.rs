//! Typed SFNT Table Directory and Table Record values.

use crate::{ByteSpan, Tag};

/// A validated record locating one top-level table in an SFNT font.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TableRecord {
    pub(crate) tag: Tag,
    pub(crate) checksum: u32,
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
    pub const fn checksum(self) -> u32 {
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

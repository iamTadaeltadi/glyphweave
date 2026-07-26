//! Domain-specific causes carried by SFNT Diagnostics.

mod cmap;
mod display;
mod layout;
mod outline;

pub use cmap::{Cmap14TableKind, CmapCause};
pub use layout::{LayoutCause, LayoutListKind, LayoutTarget};
pub use outline::{CoordinateAxis, OutlineCause};

/// The domain-specific cause of a Diagnostic.
///
/// Causes grow as later stages are implemented; the enum is `#[non_exhaustive]` so that a new
/// variant never breaks a caller that already matches it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Cause {
    /// Font Collection decoding did not find the required `ttcf` identification Tag.
    InvalidCollectionTag {
        /// The four bytes found at the beginning of the input.
        found: crate::Tag,
    },
    /// The TTC Header declared a version Glyphweave cannot interpret.
    UnsupportedCollectionVersion {
        /// The header's major version.
        major: u16,
        /// The header's minor version.
        minor: u16,
    },
    /// A TTC Header advertises no font resources.
    EmptyFontCollection,
    /// A requested Face Index is not present in the font file.
    FaceIndexOutOfBounds {
        /// The requested zero-based index.
        index: crate::FaceIndex,
        /// The number of available Faces.
        face_count: usize,
    },
    /// The advertised Face count cannot fit in the available TTC Header bytes.
    CollectionFaceCountOutOfBounds {
        /// The number of Faces declared by the header.
        count: u32,
        /// The greatest number of directory offsets available in the input.
        maximum: usize,
    },
    /// Version 2.0 DSIG fields are neither all null nor a valid aligned descriptor.
    InvalidCollectionSignatureDescriptor {
        /// The raw signature Tag field.
        tag: u32,
        /// The declared signature length.
        length: u32,
        /// The declared absolute signature offset.
        offset: u32,
    },
    /// A collection signature does not occupy the final declared byte range.
    CollectionSignatureNotAtEnd {
        /// The checked signature range.
        signature: crate::ByteSpan,
        /// The complete collection byte length.
        font_length: usize,
    },
    /// A file-level DSIG range occupies TTC Header bytes.
    CollectionSignatureOverlapsHeader {
        /// The declared signature range.
        signature: crate::ByteSpan,
        /// The TTC Header range.
        header: crate::ByteSpan,
    },
    /// A file-level DSIG range occupies a Face's Table Directory bytes.
    CollectionSignatureOverlapsFaceDirectory {
        /// The declared signature range.
        signature: crate::ByteSpan,
        /// The affected Face Index.
        face: crate::FaceIndex,
        /// The covered directory range.
        directory: crate::ByteSpan,
    },
    /// Two Face Indices point to the same Table Directory.
    DuplicateFaceDirectoryOffset {
        /// The first Face Index using the offset.
        first: crate::FaceIndex,
        /// The later Face Index reusing the offset.
        second: crate::FaceIndex,
        /// The duplicated absolute byte offset.
        offset: usize,
    },
    /// A Face's Table Directory occupies bytes belonging to the TTC Header.
    FaceDirectoryOverlapsCollectionHeader {
        /// The offending Face Index.
        index: crate::FaceIndex,
        /// The minimum range needed by the directory header.
        directory: crate::ByteSpan,
        /// The TTC Header range.
        header: crate::ByteSpan,
    },
    /// A Face offset cannot locate a complete Table Directory header.
    FaceDirectoryOutOfBounds {
        /// The affected Face Index.
        index: crate::FaceIndex,
        /// The declared absolute directory offset.
        offset: usize,
        /// The complete font-file length.
        font_length: usize,
    },
    /// Two Face table directories occupy common bytes.
    OverlappingFaceDirectories {
        /// The first directory in absolute byte order.
        first: (crate::FaceIndex, crate::ByteSpan),
        /// The next intersecting directory.
        second: (crate::FaceIndex, crate::ByteSpan),
    },
    /// A Face table occupies TTC Header bytes.
    TableOverlapsCollectionHeader {
        /// The Face that references the table.
        face: crate::FaceIndex,
        /// The offending table.
        tag: crate::Tag,
        /// The table range.
        table: crate::ByteSpan,
        /// The TTC Header range.
        header: crate::ByteSpan,
    },
    /// A table referenced by one Face occupies another Face's directory bytes.
    TableOverlapsFaceDirectory {
        /// The Face that references the table.
        face: crate::FaceIndex,
        /// The offending table.
        tag: crate::Tag,
        /// The table range.
        table: crate::ByteSpan,
        /// The Face whose directory is covered.
        directory_face: crate::FaceIndex,
        /// The covered directory range.
        directory: crate::ByteSpan,
    },
    /// A Face table occupies file-level DSIG bytes.
    TableOverlapsCollectionSignature {
        /// The Face that references the table.
        face: crate::FaceIndex,
        /// The offending table.
        tag: crate::Tag,
        /// The table range.
        table: crate::ByteSpan,
        /// The file-level DSIG range.
        signature: crate::ByteSpan,
    },
    /// Tables from different Faces partially overlap instead of sharing an exact range.
    OverlappingFaceTables {
        /// The first table in absolute byte order.
        first: (crate::FaceIndex, crate::Tag, crate::ByteSpan),
        /// The next intersecting table.
        second: (crate::FaceIndex, crate::Tag, crate::ByteSpan),
    },
    /// A character-map subtable violated a format-specific invariant.
    Cmap(CmapCause),
    /// A TrueType outline table violated an outline-specific invariant.
    Outline(OutlineCause),
    /// An OpenType layout table violated a shared-structure invariant.
    Layout(LayoutCause),
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
    /// A top-level table occupies bytes used by its Table Directory.
    TableOverlapsDirectory {
        /// The offending table.
        tag: crate::Tag,
        /// The checked table range.
        table: crate::ByteSpan,
        /// The checked directory range.
        directory: crate::ByteSpan,
    },
    /// Two distinct top-level tables cover common bytes.
    OverlappingTables {
        /// The first table in absolute byte order.
        first: (crate::Tag, crate::ByteSpan),
        /// The next table that intersects it.
        second: (crate::Tag, crate::ByteSpan),
    },
    /// A table's bytes do not match its declared Checksum.
    InvalidTableChecksum {
        /// The offending table.
        tag: crate::Tag,
        /// The Checksum stored in the Table Record.
        declared: crate::Checksum,
        /// The Checksum computed from the table bytes.
        computed: crate::Checksum,
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

#[cfg(test)]
mod cmap_tests;

#[cfg(test)]
mod outline_tests;

#[cfg(test)]
mod tests;

//! Domain-specific causes carried by SFNT Diagnostics.

use core::fmt;

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

impl Cause {
    fn fmt_collection(self, f: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        let rendered = match self {
            Self::InvalidCollectionTag { found } => {
                write!(f, "expected collection tag ttcf but found {found:?}")
            }
            Self::UnsupportedCollectionVersion { major, minor } => {
                write!(f, "unsupported collection version {major}.{minor}")
            }
            Self::EmptyFontCollection => f.write_str("font collection contains no faces"),
            Self::CollectionFaceCountOutOfBounds { count, maximum } => write!(
                f,
                "collection face count {count} exceeds available offset capacity {maximum}"
            ),
            Self::InvalidCollectionSignatureDescriptor {
                tag,
                length,
                offset,
            } => write!(
                f,
                "invalid collection signature descriptor tag=0x{tag:08x} length={length} offset={offset}"
            ),
            Self::CollectionSignatureNotAtEnd {
                signature,
                font_length,
            } => write!(
                f,
                "collection signature at {signature} must end at file length {font_length}"
            ),
            Self::DuplicateFaceDirectoryOffset {
                first,
                second,
                offset,
            } => write!(
                f,
                "face indices {first} and {second} share directory offset {offset}"
            ),
            Self::FaceDirectoryOverlapsCollectionHeader {
                index,
                directory,
                header,
            } => write!(
                f,
                "face {index} directory at {directory} overlaps collection header {header}"
            ),
            Self::OverlappingFaceDirectories { first, second } => write!(
                f,
                "face {} directory at {} overlaps face {} directory at {}",
                first.0, first.1, second.0, second.1
            ),
            Self::TableOverlapsCollectionHeader {
                face,
                tag,
                table,
                header,
            } => write!(
                f,
                "face {face} table {tag} at {table} overlaps collection header {header}"
            ),
            Self::TableOverlapsFaceDirectory {
                face,
                tag,
                table,
                directory_face,
                directory,
            } => write!(
                f,
                "face {face} table {tag} at {table} overlaps face {directory_face} directory {directory}"
            ),
            Self::TableOverlapsCollectionSignature {
                face,
                tag,
                table,
                signature,
            } => write!(
                f,
                "face {face} table {tag} at {table} overlaps collection signature {signature}"
            ),
            Self::OverlappingFaceTables { first, second } => write!(
                f,
                "face {} table {} at {} overlaps face {} table {} at {}",
                first.0, first.1, first.2, second.0, second.1, second.2
            ),
            _ => return None,
        };
        Some(rendered)
    }
}

impl fmt::Display for Cause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(rendered) = self.fmt_collection(f) {
            return rendered;
        }
        match *self {
            Self::InvalidCollectionTag { .. }
            | Self::UnsupportedCollectionVersion { .. }
            | Self::EmptyFontCollection
            | Self::CollectionFaceCountOutOfBounds { .. }
            | Self::InvalidCollectionSignatureDescriptor { .. }
            | Self::CollectionSignatureNotAtEnd { .. }
            | Self::DuplicateFaceDirectoryOffset { .. }
            | Self::FaceDirectoryOverlapsCollectionHeader { .. }
            | Self::OverlappingFaceDirectories { .. }
            | Self::TableOverlapsCollectionHeader { .. }
            | Self::TableOverlapsFaceDirectory { .. }
            | Self::TableOverlapsCollectionSignature { .. }
            | Self::OverlappingFaceTables { .. } => unreachable!(),
            Self::UnsupportedSfntVersion { raw } => {
                write!(f, "unsupported SFNT version 0x{raw:08x}")
            }
            Self::UnrepresentableTableCount { count, maximum } => {
                write!(
                    f,
                    "table count {count} exceeds representable maximum {maximum}"
                )
            }
            Self::InvalidSearchParameters { expected, actual } => write!(
                f,
                "directory search parameters {actual:?} do not match derived values {expected:?}"
            ),
            Self::TableOutOfBounds {
                tag,
                offset,
                length,
                font_length,
            } => write!(
                f,
                "table {tag} at {offset}+{length} exceeds font length {font_length}"
            ),
            Self::InvalidTableTag { tag } => write!(f, "invalid table tag {tag:?}"),
            Self::DuplicateTableTag { tag } => write!(f, "duplicate table tag {tag}"),
            Self::UnsortedTableTags { previous, current } => {
                write!(f, "table tag {current} follows larger tag {previous}")
            }
            Self::MisalignedTable { tag, offset } => {
                write!(f, "table {tag} starts at unaligned offset {offset}")
            }
            Self::TableOverlapsDirectory {
                tag,
                table,
                directory,
            } => write!(f, "table {tag} at {table} overlaps directory {directory}"),
            Self::OverlappingTables { first, second } => write!(
                f,
                "table {} at {} overlaps table {} at {}",
                first.0, first.1, second.0, second.1
            ),
            Self::InvalidTableChecksum {
                tag,
                declared,
                computed,
            } => write!(
                f,
                "table {tag} checksum 0x{:08x} does not match 0x{:08x}",
                declared.raw(),
                computed.raw()
            ),
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
mod tests;

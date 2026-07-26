//! Human-readable rendering for typed Diagnostic causes.

use core::fmt;

use super::Cause;

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
            Self::FaceIndexOutOfBounds { index, face_count } => write!(
                f,
                "face index {index} is out of bounds for {face_count} face(s)"
            ),
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
            Self::CollectionSignatureOverlapsHeader { signature, header } => write!(
                f,
                "collection signature at {signature} overlaps collection header {header}"
            ),
            Self::CollectionSignatureOverlapsFaceDirectory {
                signature,
                face,
                directory,
            } => write!(
                f,
                "collection signature at {signature} overlaps face {face} directory {directory}"
            ),
            _ => return self.fmt_collection_overlap(f),
        };
        Some(rendered)
    }

    fn fmt_collection_overlap(self, f: &mut fmt::Formatter<'_>) -> Option<fmt::Result> {
        let rendered = match self {
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
            Self::FaceDirectoryOutOfBounds {
                index,
                offset,
                font_length,
            } => write!(
                f,
                "face {index} directory at {offset} exceeds font length {font_length}"
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
            | Self::FaceIndexOutOfBounds { .. }
            | Self::CollectionFaceCountOutOfBounds { .. }
            | Self::InvalidCollectionSignatureDescriptor { .. }
            | Self::CollectionSignatureNotAtEnd { .. }
            | Self::CollectionSignatureOverlapsHeader { .. }
            | Self::CollectionSignatureOverlapsFaceDirectory { .. }
            | Self::DuplicateFaceDirectoryOffset { .. }
            | Self::FaceDirectoryOverlapsCollectionHeader { .. }
            | Self::FaceDirectoryOutOfBounds { .. }
            | Self::OverlappingFaceDirectories { .. }
            | Self::TableOverlapsCollectionHeader { .. }
            | Self::TableOverlapsFaceDirectory { .. }
            | Self::TableOverlapsCollectionSignature { .. }
            | Self::OverlappingFaceTables { .. } => unreachable!(),
            Self::Cmap(cause) => cause.fmt(f),
            Self::UnsupportedSfntVersion { raw } => {
                write!(f, "unsupported SFNT version 0x{raw:08x}")
            }
            Self::UnrepresentableTableCount { count, maximum } => write!(
                f,
                "table count {count} exceeds representable maximum {maximum}"
            ),
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
            Self::SpanOutOfBounds { end, length } => write!(
                f,
                "span ends at {end} but only {length} byte(s) are available"
            ),
        }
    }
}

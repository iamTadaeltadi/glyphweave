//! Public Table Directory decoding scenarios.

use glyphweave_sfnt::{SfntVersion, TableDirectory};

#[test]
fn decodes_empty_truetype_directory() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, // TrueType
        0x00, 0x00, // numTables
        0x00, 0x00, // searchRange
        0x00, 0x00, // entrySelector
        0x00, 0x00, // rangeShift
    ];
    let directory = TableDirectory::decode(&bytes, 0).unwrap();
    assert_eq!(directory.version(), SfntVersion::TrueType);
    assert_eq!((directory.span().start(), directory.span().end()), (0, 12));
    assert!(directory.is_empty());
}

#[test]
fn decodes_cff_directory_at_absolute_offset() {
    let bytes = [
        0xff, 0xff, 0xff, 0xff, // collection-owned prefix
        b'O', b'T', b'T', b'O', 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let directory = TableDirectory::decode(&bytes, 4).unwrap();
    assert_eq!(directory.version(), SfntVersion::OpenTypeCff);
    assert_eq!((directory.span().start(), directory.span().end()), (4, 16));
}

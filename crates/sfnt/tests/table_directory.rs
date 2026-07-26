//! Public Table Directory decoding scenarios.

use glyphweave_sfnt::{Cause, SfntVersion, TableDirectory, Tag};

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

#[test]
fn decodes_table_record_fields_and_spans() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, // TrueType
        0x00, 0x01, // numTables
        0x00, 0x10, // searchRange
        0x00, 0x00, // entrySelector
        0x00, 0x00, // rangeShift
        b'h', b'e', b'a', b'd', // tag
        0x12, 0x34, 0x56, 0x78, // checksum
        0x00, 0x00, 0x00, 0x1c, // offset 28
        0x00, 0x00, 0x00, 0x04, // length 4
        0xde, 0xad, 0xbe, 0xef, // table bytes
    ];
    let directory = TableDirectory::decode(&bytes, 0).unwrap();
    let record = directory.record(Tag::new(*b"head")).unwrap();
    assert_eq!(record.checksum(), 0x1234_5678);
    assert_eq!(
        (record.table_span().start(), record.table_span().end()),
        (28, 32)
    );
    assert_eq!(
        (
            record.directory_span().start(),
            record.directory_span().end()
        ),
        (12, 28)
    );
}

#[test]
fn rejects_unknown_sfnt_version_at_version_span() {
    let bytes = *b"true\0\0\0\0\0\0\0\0";
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsupportedSfntVersion {
            raw: u32::from_be_bytes(*b"true")
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "0..4");
}

#[test]
fn rejects_search_fields_not_derived_from_count() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, // TrueType
        0x00, 0x03, // numTables
        0x00, 0x10, // wrong searchRange
        0x00, 0x00, // wrong entrySelector
        0x00, 0x20, // rangeShift
    ];
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidSearchParameters {
            expected: [32, 1, 16],
            actual: [16, 0, 32],
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "6..12");
}

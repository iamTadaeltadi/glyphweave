//! Public Table Directory decoding scenarios.

use glyphweave_sfnt::{Cause, SfntVersion, TableDirectory, Tag};

fn directory_with_tags(first: [u8; 4], second: [u8; 4]) -> Vec<u8> {
    let mut bytes = vec![0x00, 0x01, 0x00, 0x00, 0, 2, 0, 32, 0, 1, 0, 0];
    for tag in [first, second] {
        bytes.extend_from_slice(&tag);
        bytes.extend_from_slice(&0_u32.to_be_bytes());
        bytes.extend_from_slice(&44_u32.to_be_bytes());
        bytes.extend_from_slice(&0_u32.to_be_bytes());
    }
    bytes
}

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

#[test]
fn rejects_count_that_cannot_encode_search_fields() {
    let bytes = [0x00, 0x01, 0x00, 0x00, 0x10, 0x00];
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::UnrepresentableTableCount {
            count: 4_096,
            maximum: 4_095,
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "4..6");
}

#[test]
fn rejects_record_whose_table_exceeds_font() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, 0, 1, 0, 16, 0, 0, 0, 0, // header
        b'g', b'l', b'y', b'f', 0, 0, 0, 0, // tag and checksum
        0, 0, 0, 28, 0, 0, 0, 5, // offset and length
    ];
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::TableOutOfBounds {
            tag: Tag::new(*b"glyf"),
            offset: 28,
            length: 5,
            font_length: 28,
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "12..28");
}

#[test]
fn rejects_non_trailing_space_inside_table_tag() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, 0, 1, 0, 16, 0, 0, 0, 0, // header
        b'a', b' ', b'b', b' ', 0, 0, 0, 0, // tag and checksum
        0, 0, 0, 28, 0, 0, 0, 0, // empty table at EOF
    ];
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidTableTag {
            tag: Tag::new(*b"a b ")
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "12..16");
}

#[test]
fn rejects_duplicate_table_tags() {
    let bytes = directory_with_tags(*b"cmap", *b"cmap");
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::DuplicateTableTag {
            tag: Tag::new(*b"cmap")
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "28..32");
}

#[test]
fn rejects_descending_table_tags() {
    let bytes = directory_with_tags(*b"head", *b"cmap");
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsortedTableTags {
            previous: Tag::new(*b"head"),
            current: Tag::new(*b"cmap"),
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "28..32");
}

#[test]
fn rejects_table_not_aligned_to_four_byte_boundary() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, 0, 1, 0, 16, 0, 0, 0, 0, // header
        b'h', b'e', b'a', b'd', 0, 0, 0, 0, // tag and checksum
        0, 0, 0, 29, 0, 0, 0, 0, // empty table at offset 29
        0,
    ];
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::MisalignedTable {
            tag: Tag::new(*b"head"),
            offset: 29,
        }
    );
    assert_eq!(diagnostic.span().unwrap().to_string(), "20..24");
}

#[test]
fn rejects_table_bytes_inside_directory_records() {
    let bytes = [
        0x00, 0x01, 0x00, 0x00, 0, 1, 0, 16, 0, 0, 0, 0, // header
        b'c', b'm', b'a', b'p', 0, 0, 0, 0, // tag and checksum
        0, 0, 0, 24, 0, 0, 0, 4, // table covers final length field
    ];
    let diagnostic = TableDirectory::decode(&bytes, 0).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::TableOverlapsDirectory {
            tag: Tag::new(*b"cmap"),
            table: glyphweave_sfnt::ByteSpan::between(24, 28).unwrap(),
            directory: glyphweave_sfnt::ByteSpan::between(0, 28).unwrap(),
        }
    );
}

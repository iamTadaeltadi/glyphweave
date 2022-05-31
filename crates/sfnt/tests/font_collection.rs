//! Integration tests for bounded Font Collection decoding and Face selection.

use glyphweave_sfnt::{
    ByteSpan, Cause, CollectionVersion, Face, FaceIndex, FontCollection, SfntVersion, Tag,
};

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_empty_directory(bytes: &mut Vec<u8>, version: u32) {
    push_u32(bytes, version);
    bytes.extend_from_slice(&[0; 8]);
}

#[test]
fn decodes_version_one_collection() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 16);
    push_empty_directory(&mut bytes, 0x0001_0000);

    let collection = FontCollection::decode(&bytes).unwrap();
    let face = collection.face(FaceIndex::FIRST).unwrap();
    assert_eq!(collection.version(), CollectionVersion::Version1);
    assert_eq!(collection.header_span().to_string(), "0..16");
    assert_eq!(face.directory_offset(), 16);
    assert_eq!(face.directory().version(), SfntVersion::TrueType);
}

#[test]
fn decodes_unsigned_version_two_collection() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 28);
    bytes.extend_from_slice(&[0; 12]);
    push_empty_directory(&mut bytes, u32::from_be_bytes(*b"OTTO"));

    let collection = FontCollection::decode(&bytes).unwrap();
    assert_eq!(collection.version(), CollectionVersion::Version2);
    assert_eq!(collection.header_span().to_string(), "0..28");
    assert_eq!(collection.len(), 1);
    assert_eq!(collection.signature(), None);
}

#[test]
fn decodes_version_two_signature_descriptor() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 28);
    push_u32(&mut bytes, u32::from_be_bytes(*b"DSIG"));
    push_u32(&mut bytes, 8);
    push_u32(&mut bytes, 40);
    push_empty_directory(&mut bytes, 0x0001_0000);
    bytes.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0]);

    let collection = FontCollection::decode(&bytes).unwrap();
    let signature = collection.signature().unwrap();
    assert_eq!(signature.span().to_string(), "40..48");
    assert_eq!(signature.bytes(&bytes).unwrap(), &bytes[40..]);
}

#[test]
fn rejects_non_collection_identification_tag() {
    let diagnostic = FontCollection::decode(b"OTTO-not-a-collection").unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCollectionTag {
            found: Tag::new(*b"OTTO")
        }
    );
}

#[test]
fn rejects_unknown_collection_header_version() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 1);
    push_u32(&mut bytes, 1);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsupportedCollectionVersion { major: 2, minor: 1 }
    );
}

#[test]
fn rejects_collection_without_faces() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 0);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(8, 4));
    assert_eq!(diagnostic.cause(), Cause::EmptyFontCollection);
}

#[test]
fn rejects_face_count_larger_than_available_header() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 28);
    bytes.extend_from_slice(&[0; 12]);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(8, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::CollectionFaceCountOutOfBounds {
            count: 2,
            maximum: 1
        }
    );
}

#[test]
fn rejects_incoherent_signature_descriptor_fields() {
    let cases = [
        (0, 0, 40),
        (u32::from_be_bytes(*b"BAD!"), 8, 40),
        (u32::from_be_bytes(*b"DSIG"), 0, 40),
        (u32::from_be_bytes(*b"DSIG"), 8, 41),
    ];
    for (tag, length, offset) in cases {
        let mut bytes = b"ttcf".to_vec();
        push_u16(&mut bytes, 2);
        push_u16(&mut bytes, 0);
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 28);
        push_u32(&mut bytes, tag);
        push_u32(&mut bytes, length);
        push_u32(&mut bytes, offset);

        let diagnostic = FontCollection::decode(&bytes).unwrap_err();
        assert_eq!(diagnostic.span(), ByteSpan::sized(16, 12));
        assert_eq!(
            diagnostic.cause(),
            Cause::InvalidCollectionSignatureDescriptor {
                tag,
                length,
                offset
            }
        );
    }
}

#[test]
fn rejects_signature_range_that_is_not_the_file_tail() {
    for (offset, length) in [(40, 8), (48, 8)] {
        let mut bytes = b"ttcf".to_vec();
        push_u16(&mut bytes, 2);
        push_u16(&mut bytes, 0);
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 28);
        push_u32(&mut bytes, u32::from_be_bytes(*b"DSIG"));
        push_u32(&mut bytes, length);
        push_u32(&mut bytes, offset);
        bytes.resize(52, 0);

        let diagnostic = FontCollection::decode(&bytes).unwrap_err();
        let signature = ByteSpan::sized(offset as usize, length as usize).unwrap();
        assert_eq!(diagnostic.span(), Some(signature));
        assert_eq!(
            diagnostic.cause(),
            Cause::CollectionSignatureNotAtEnd {
                signature,
                font_length: 52
            }
        );
    }
}

#[test]
fn rejects_duplicate_face_directory_offsets() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 20);
    push_empty_directory(&mut bytes, 0x0001_0000);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::DuplicateFaceDirectoryOffset {
            first: FaceIndex::new(0),
            second: FaceIndex::new(1),
            offset: 20
        }
    );
}

#[test]
fn rejects_face_directory_inside_collection_header() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 4);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(12, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::FaceDirectoryOverlapsCollectionHeader {
            index: FaceIndex::FIRST,
            directory: ByteSpan::sized(4, 12).unwrap(),
            header: ByteSpan::sized(0, 16).unwrap()
        }
    );
}

#[test]
fn rejects_overlapping_face_directories() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 32);
    push_u32(&mut bytes, 0x0001_0000);
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 16);
    push_u16(&mut bytes, 0);
    push_u16(&mut bytes, 0);
    bytes.extend_from_slice(b"OTTO");
    bytes.extend_from_slice(&[0; 12]);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(20, 28));
    assert_eq!(
        diagnostic.cause(),
        Cause::OverlappingFaceDirectories {
            first: (FaceIndex::new(0), ByteSpan::sized(20, 28).unwrap()),
            second: (FaceIndex::new(1), ByteSpan::sized(32, 12).unwrap())
        }
    );
}

#[test]
fn rejects_face_table_inside_collection_header() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 0x0001_0000);
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 16);
    push_u16(&mut bytes, 0);
    push_u16(&mut bytes, 0);
    bytes.extend_from_slice(b"name");
    push_u32(&mut bytes, u32::from_be_bytes(*b"ttcf"));
    push_u32(&mut bytes, 0);
    push_u32(&mut bytes, 4);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::TableOverlapsCollectionHeader {
            face: FaceIndex::FIRST,
            tag: Tag::new(*b"name"),
            table: ByteSpan::sized(0, 4).unwrap(),
            header: ByteSpan::sized(0, 16).unwrap()
        }
    );
}

#[test]
fn rejects_table_covering_another_face_directory() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 48);
    push_u32(&mut bytes, 0x0001_0000);
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 16);
    push_u16(&mut bytes, 0);
    push_u16(&mut bytes, 0);
    bytes.extend_from_slice(b"name");
    push_u32(&mut bytes, 0x0001_0000);
    push_u32(&mut bytes, 48);
    push_u32(&mut bytes, 12);
    push_empty_directory(&mut bytes, 0x0001_0000);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(48, 12));
    assert_eq!(
        diagnostic.cause(),
        Cause::TableOverlapsFaceDirectory {
            face: FaceIndex::FIRST,
            tag: Tag::new(*b"name"),
            table: ByteSpan::sized(48, 12).unwrap(),
            directory_face: FaceIndex::new(1),
            directory: ByteSpan::sized(48, 12).unwrap()
        }
    );
}

#[test]
fn rejects_face_table_covering_collection_signature() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 28);
    push_u32(&mut bytes, u32::from_be_bytes(*b"DSIG"));
    push_u32(&mut bytes, 8);
    push_u32(&mut bytes, 56);
    push_u32(&mut bytes, 0x0001_0000);
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 16);
    push_u16(&mut bytes, 0);
    push_u16(&mut bytes, 0);
    bytes.extend_from_slice(b"name");
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 56);
    push_u32(&mut bytes, 8);
    bytes.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0]);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(56, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::TableOverlapsCollectionSignature {
            face: FaceIndex::FIRST,
            tag: Tag::new(*b"name"),
            table: ByteSpan::sized(56, 8).unwrap(),
            signature: ByteSpan::sized(56, 8).unwrap()
        }
    );
}

#[test]
fn rejects_partially_overlapping_tables_across_faces() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 48);
    for offset in [76, 80] {
        push_u32(&mut bytes, 0x0001_0000);
        push_u16(&mut bytes, 1);
        push_u16(&mut bytes, 16);
        push_u16(&mut bytes, 0);
        push_u16(&mut bytes, 0);
        bytes.extend_from_slice(b"name");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, offset);
        push_u32(&mut bytes, 8);
    }
    bytes.extend_from_slice(&[0; 12]);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(76, 12));
    assert_eq!(
        diagnostic.cause(),
        Cause::OverlappingFaceTables {
            first: (
                FaceIndex::new(0),
                Tag::new(*b"name"),
                ByteSpan::sized(76, 8).unwrap()
            ),
            second: (
                FaceIndex::new(1),
                Tag::new(*b"name"),
                ByteSpan::sized(80, 8).unwrap()
            )
        }
    );
}

#[test]
fn permits_exact_table_sharing_across_faces() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 48);
    for _ in 0..2 {
        push_u32(&mut bytes, 0x0001_0000);
        push_u16(&mut bytes, 1);
        push_u16(&mut bytes, 16);
        push_u16(&mut bytes, 0);
        push_u16(&mut bytes, 0);
        bytes.extend_from_slice(b"name");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 76);
        push_u32(&mut bytes, 8);
    }
    bytes.extend_from_slice(&[0; 8]);

    let collection = FontCollection::decode(&bytes).unwrap();
    let first = collection.face(FaceIndex::new(0)).unwrap();
    let second = collection.face(FaceIndex::new(1)).unwrap();
    assert_eq!(
        first.record(Tag::new(*b"name")).unwrap().table_span(),
        second.record(Tag::new(*b"name")).unwrap().table_span()
    );
}

#[test]
fn preserves_face_order_when_directory_offsets_descend() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 32);
    push_u32(&mut bytes, 20);
    push_empty_directory(&mut bytes, 0x0001_0000);
    push_empty_directory(&mut bytes, u32::from_be_bytes(*b"OTTO"));

    let collection = FontCollection::decode(&bytes).unwrap();
    assert_eq!(
        collection
            .face(FaceIndex::new(0))
            .unwrap()
            .directory_offset(),
        32
    );
    assert_eq!(
        collection
            .face(FaceIndex::new(1))
            .unwrap()
            .directory_offset(),
        20
    );
    assert_eq!(collection.faces()[0].index(), FaceIndex::new(0));
    assert_eq!(collection.faces()[1].index(), FaceIndex::new(1));
}

#[test]
fn selects_first_face_from_standalone_font() {
    let mut bytes = Vec::new();
    push_empty_directory(&mut bytes, 0x0001_0000);

    let face = Face::select(&bytes, FaceIndex::FIRST).unwrap();
    assert_eq!(face.index(), FaceIndex::FIRST);
    assert_eq!(face.directory_offset(), 0);
    assert_eq!(face.directory().version(), SfntVersion::TrueType);
}

#[test]
fn rejects_nonzero_face_index_for_standalone_font() {
    let mut bytes = Vec::new();
    push_empty_directory(&mut bytes, 0x0001_0000);

    let diagnostic = Face::select(&bytes, FaceIndex::new(1)).unwrap_err();
    assert_eq!(diagnostic.span(), None);
    assert_eq!(
        diagnostic.cause(),
        Cause::FaceIndexOutOfBounds {
            index: FaceIndex::new(1),
            face_count: 1
        }
    );
}

#[test]
fn selects_requested_face_from_collection() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 32);
    push_empty_directory(&mut bytes, 0x0001_0000);
    push_empty_directory(&mut bytes, u32::from_be_bytes(*b"OTTO"));

    let face = Face::select(&bytes, FaceIndex::new(1)).unwrap();
    assert_eq!(face.index(), FaceIndex::new(1));
    assert_eq!(face.directory_offset(), 32);
    assert_eq!(face.directory().version(), SfntVersion::OpenTypeCff);
}

#[test]
fn rejects_missing_collection_face_index() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 16);
    push_empty_directory(&mut bytes, 0x0001_0000);

    let diagnostic = Face::select(&bytes, FaceIndex::new(2)).unwrap_err();
    assert_eq!(diagnostic.span(), None);
    assert_eq!(
        diagnostic.cause(),
        Cause::FaceIndexOutOfBounds {
            index: FaceIndex::new(2),
            face_count: 1
        }
    );
}

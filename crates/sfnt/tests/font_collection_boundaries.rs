//! Boundary and eager-validation tests for OpenType Font Collections.

use glyphweave_sfnt::{ByteSpan, Cause, Face, FaceIndex, FontCollection, Tag};

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
fn rejects_face_offset_without_complete_directory_header() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 20);
    bytes.resize(24, 0);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(12, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::FaceDirectoryOutOfBounds {
            index: FaceIndex::FIRST,
            offset: 20,
            font_length: 24
        }
    );
}

#[test]
fn rejects_malformed_unselected_face() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 1);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 2);
    push_u32(&mut bytes, 20);
    push_u32(&mut bytes, 32);
    push_empty_directory(&mut bytes, 0x0001_0000);
    push_empty_directory(&mut bytes, u32::from_be_bytes(*b"BAD!"));

    let diagnostic = Face::select(&bytes, FaceIndex::FIRST).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(32, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsupportedSfntVersion {
            raw: u32::from_be_bytes(*b"BAD!")
        }
    );
}

#[test]
fn rejects_every_truncated_prefix_of_version_two_collection() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 28);
    bytes.extend_from_slice(&[0; 12]);
    push_empty_directory(&mut bytes, 0x0001_0000);

    for length in 0..bytes.len() {
        assert!(
            FontCollection::decode(&bytes[..length]).is_err(),
            "prefix of length {length} unexpectedly decoded"
        );
    }
    assert!(FontCollection::decode(&bytes).is_ok());
}

#[test]
fn rejects_signature_covering_collection_header() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 28);
    push_u32(&mut bytes, u32::from_be_bytes(*b"DSIG"));
    push_u32(&mut bytes, 32);
    push_u32(&mut bytes, 16);
    push_empty_directory(&mut bytes, 0x0001_0000);
    bytes.resize(48, 0);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 48));
    assert_eq!(
        diagnostic.cause(),
        Cause::CollectionSignatureOverlapsHeader {
            signature: ByteSpan::sized(16, 32).unwrap(),
            header: ByteSpan::sized(0, 28).unwrap()
        }
    );
}

#[test]
fn rejects_signature_covering_face_directory() {
    let mut bytes = b"ttcf".to_vec();
    push_u16(&mut bytes, 2);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 1);
    push_u32(&mut bytes, 28);
    push_u32(&mut bytes, u32::from_be_bytes(*b"DSIG"));
    push_u32(&mut bytes, 16);
    push_u32(&mut bytes, 32);
    push_empty_directory(&mut bytes, 0x0001_0000);
    bytes.resize(48, 0);

    let diagnostic = FontCollection::decode(&bytes).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::sized(28, 20));
    assert_eq!(
        diagnostic.cause(),
        Cause::CollectionSignatureOverlapsFaceDirectory {
            signature: ByteSpan::sized(32, 16).unwrap(),
            face: FaceIndex::FIRST,
            directory: ByteSpan::sized(28, 12).unwrap()
        }
    );
}

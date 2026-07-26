//! Integration tests for bounded Font Collection decoding and Face selection.

use glyphweave_sfnt::{
    ByteSpan, Cause, CollectionVersion, FaceIndex, FontCollection, SfntVersion, Tag,
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

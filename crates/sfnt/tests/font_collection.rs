//! Integration tests for bounded Font Collection decoding and Face selection.

use glyphweave_sfnt::{CollectionVersion, FaceIndex, FontCollection, SfntVersion};

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

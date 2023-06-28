//! Semantic decoding tests for OpenType Script tables.

use glyphweave_layout::Script;
use glyphweave_sfnt::{ByteSpan, Tag};

const DEU: Tag = Tag::new(*b"DEU ");
const FRA: Tag = Tag::new(*b"FRA ");

#[test]
fn decodes_default_and_named_language_systems() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x10, 0x00, 0x02, // defaultLangSys, langSysCount
        b'D', b'E', b'U', b' ', 0x00, 0x16, // DEU record
        b'F', b'R', b'A', b' ', 0x00, 0x1e, // FRA record
        0x00, 0x00, 0xff, 0xff, 0x00, 0x00, // default language
        0x00, 0x00, 0x00, 0x03, 0x00, 0x01, 0x00, 0x09, // DEU
        0x00, 0x00, 0xff, 0xff, 0x00, 0x02, 0x00, 0x07, 0x00, 0x02, // FRA
    ];
    let script = Script::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(script.default_language().unwrap().required_feature(), None);
    assert_eq!(script.language(DEU).unwrap().required_feature(), Some(3));
    assert_eq!(script.language(DEU).unwrap().feature_indices(), &[9]);
    assert_eq!(script.language(FRA).unwrap().feature_indices(), &[2, 7]);
}

#[test]
fn decodes_script_without_default_or_named_languages() {
    let bytes = [0x00, 0x00, 0x00, 0x00];
    let script = Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();
    assert!(script.is_empty());
}

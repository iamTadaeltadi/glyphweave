//! Semantic decoding tests for OpenType Script Lists.

use glyphweave_layout::ScriptList;
use glyphweave_sfnt::{ByteSpan, Tag};

const ARAB: Tag = Tag::new(*b"arab");
const LATN: Tag = Tag::new(*b"latn");

#[test]
fn decodes_nested_scripts_and_languages() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x02, // scriptCount
        b'a', b'r', b'a', b'b', 0x00, 0x0e, // arab
        b'l', b'a', b't', b'n', 0x00, 0x12, // latn
        0x00, 0x00, 0x00, 0x00, // empty arab Script
        0x00, 0x00, 0x00, 0x01, // latn Script header
        b'D', b'E', b'U', b' ', 0x00, 0x0a, // named language record
        0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x05, // Language System
    ];
    let list = ScriptList::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert!(list.script(ARAB).unwrap().is_empty());
    let language = list
        .script(LATN)
        .unwrap()
        .language(Tag::new(*b"DEU "))
        .unwrap();
    assert_eq!(language.required_feature(), Some(2));
    assert_eq!(language.feature_indices(), &[5]);
}

#[test]
fn decodes_empty_script_list() {
    let bytes = [0x00, 0x00];
    let list = ScriptList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();
    assert!(list.is_empty());
}

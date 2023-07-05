//! Semantic decoding tests for OpenType Script Lists.

use glyphweave_layout::ScriptList;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutListKind, LayoutTarget, Tag};

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

#[test]
fn rejects_malformed_script_tag() {
    let bytes = [0x00, 0x01, b' ', b' ', b' ', b' ', 0x00, 0x08];
    let diagnostic =
        ScriptList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(2, 6));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutTag {
            list: LayoutListKind::Script,
            tag: Tag::new(*b"    "),
        })
    );
}

#[test]
fn rejects_decreasing_script_tags() {
    let bytes = [
        0x00, 0x02, b'l', b'a', b't', b'n', 0x00, 0x0e, b'a', b'r', b'a', b'b', 0x00, 0x0e,
    ];
    let diagnostic =
        ScriptList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(8, 12));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsortedLayoutTags {
            list: LayoutListKind::Script,
            previous: LATN,
            current: ARAB,
        })
    );
}

#[test]
fn rejects_duplicate_script_tags() {
    let bytes = [
        0x00, 0x02, b'l', b'a', b't', b'n', 0x00, 0x0e, b'l', b'a', b't', b'n', 0x00, 0x0e,
    ];
    let diagnostic =
        ScriptList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::DuplicateLayoutTag {
            list: LayoutListKind::Script,
            tag: LATN,
        })
    );
}

#[test]
fn rejects_script_target_inside_records() {
    let bytes = [0x00, 0x01, b'l', b'a', b't', b'n', 0x00, 0x02];
    let diagnostic =
        ScriptList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(6, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::Script,
            offset: 2,
            minimum: 8,
            container_length: 8,
        })
    );
}

#[test]
fn accepts_exact_script_target_aliasing() {
    let bytes = [
        0x00, 0x02, b'a', b'r', b'a', b'b', 0x00, 0x0e, b'l', b'a', b't', b'n', 0x00, 0x0e, 0x00,
        0x00, 0x00, 0x00,
    ];
    let list = ScriptList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(list.script(ARAB), list.script(LATN));
    assert!(list.script(LATN).unwrap().is_empty());
}

#[test]
fn every_strict_script_list_prefix_is_rejected() {
    let complete = [
        0x00, 0x01, b'l', b'a', b't', b'n', 0x00, 0x08, // record
        0x00, 0x00, 0x00, 0x00, // Script
    ];
    for end in 0..complete.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(
            ScriptList::decode(&complete, span).is_err(),
            "strict prefix {end} decoded"
        );
    }
}

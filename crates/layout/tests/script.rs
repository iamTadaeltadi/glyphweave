//! Semantic decoding tests for OpenType Script tables.

use glyphweave_layout::Script;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutListKind, LayoutTarget, Tag};

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

#[test]
fn rejects_malformed_language_tag() {
    let bytes = [
        0x00, 0x00, 0x00, 0x01, b' ', b' ', b' ', b' ', 0x00, 0x0a, 0x00, 0x00,
    ];
    let diagnostic =
        Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(4, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutTag {
            list: LayoutListKind::Language,
            tag: Tag::new(*b"    "),
        })
    );
}

#[test]
fn rejects_decreasing_language_tags() {
    let bytes = [
        0x00, 0x00, 0x00, 0x02, b'F', b'R', b'A', b' ', 0x00, 0x10, b'D', b'E', b'U', b' ', 0x00,
        0x10,
    ];
    let diagnostic =
        Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(10, 14));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsortedLayoutTags {
            list: LayoutListKind::Language,
            previous: FRA,
            current: DEU,
        })
    );
}

#[test]
fn rejects_duplicate_language_tags() {
    let bytes = [
        0x00, 0x00, 0x00, 0x02, b'D', b'E', b'U', b' ', 0x00, 0x10, b'D', b'E', b'U', b' ', 0x00,
        0x10,
    ];
    let diagnostic =
        Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::DuplicateLayoutTag {
            list: LayoutListKind::Language,
            tag: DEU,
        })
    );
}

#[test]
fn rejects_default_target_inside_language_records() {
    let bytes = [
        0x00, 0x04, 0x00, 0x01, b'D', b'E', b'U', b' ', 0x00, 0x0a, 0x00, 0x00,
    ];
    let diagnostic =
        Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::DefaultLanguageSystem,
            offset: 4,
            minimum: 10,
            container_length: 12,
        })
    );
}

#[test]
fn rejects_null_named_language_target() {
    let bytes = [0x00, 0x00, 0x00, 0x01, b'D', b'E', b'U', b' ', 0x00, 0x00];
    let diagnostic =
        Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(8, 10));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::LanguageSystem,
            offset: 0,
            minimum: 10,
            container_length: 10,
        })
    );
}

#[test]
fn rejects_named_target_at_container_end() {
    let bytes = [0x00, 0x00, 0x00, 0x01, b'D', b'E', b'U', b' ', 0x00, 0x0a];
    let diagnostic =
        Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::LanguageSystem,
            offset: 10,
            minimum: 10,
            container_length: 10,
        })
    );
}

#[test]
fn accepts_exact_language_target_aliasing() {
    let bytes = [
        0x00, 0x0a, 0x00, 0x01, b'D', b'E', b'U', b' ', 0x00, 0x0a, // shared target
        0x00, 0x00, 0x00, 0x04, 0x00, 0x00, // Language System
    ];
    let script = Script::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(script.default_language(), script.language(DEU));
    assert_eq!(script.language(DEU).unwrap().required_feature(), Some(4));
}

#[test]
fn every_strict_script_prefix_is_rejected() {
    let complete = [
        0x00, 0x00, 0x00, 0x01, b'D', b'E', b'U', b' ', 0x00, 0x0a, // record
        0x00, 0x00, 0xff, 0xff, 0x00, 0x00, // Language System
    ];
    for end in 0..complete.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(
            Script::decode(&complete, span).is_err(),
            "strict prefix {end} decoded"
        );
    }
}

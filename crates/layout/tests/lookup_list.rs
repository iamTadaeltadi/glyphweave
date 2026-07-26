//! Semantic decoding tests for OpenType LookupList tables.

use glyphweave_layout::LookupList;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTableKind, LayoutTarget};

#[test]
fn decodes_lookup_index_and_processing_order() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x02, 0x00, 0x06, 0x00, 0x0f, // LookupList
        0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0xfe, // lookup zero
        0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0xfd, // lookup one
    ];
    let list = LookupList::decode(
        &bytes,
        ByteSpan::between(2, bytes.len()).unwrap(),
        LayoutTableKind::Positioning,
    )
    .unwrap();

    assert_eq!(list.table(), Some(LayoutTableKind::Positioning));
    assert_eq!(list.lookup(0).unwrap().lookup_type(), 1);
    assert_eq!(list.lookup(1).unwrap().lookup_type(), 3);
    assert_eq!(
        list.lookup(0).unwrap().subtables()[0],
        ByteSpan::between(16, 26).unwrap()
    );
    assert_eq!(
        list.lookup(1).unwrap().subtables()[0],
        ByteSpan::between(25, 26).unwrap()
    );
}

#[test]
fn decodes_empty_lookup_list() {
    let bytes = [0x00, 0x00];
    let list = LookupList::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap();
    assert!(list.is_empty());
    assert_eq!(list.table(), None);
}

#[test]
fn rejects_lookup_target_inside_offset_array() {
    let bytes = [0x00, 0x01, 0x00, 0x02];
    let diagnostic = LookupList::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(2, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::Lookup,
            offset: 2,
            minimum: 4,
            container_length: 4,
        })
    );
}

#[test]
fn accepts_exact_lookup_target_aliasing() {
    let bytes = [
        0x00, 0x02, 0x00, 0x06, 0x00, 0x06, // two aliases
        0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x08, 0xfe,
    ];
    let list = LookupList::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap();

    assert_eq!(list.lookup(0), list.lookup(1));
}

//! Semantic decoding tests for shared OpenType Lookup tables.

use glyphweave_layout::{Lookup, MarkFilter};
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTableKind, LayoutTarget};

#[test]
fn decodes_flags_mark_set_and_ordered_subtables() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x03, // GPOS cursive attachment
        0x09, 0x17, // class 9, filtering set, RTL, ignore base + ligatures
        0x00, 0x02, // subTableCount
        0x00, 0x0c, 0x00, 0x0e, // subtable offsets
        0x00, 0x05, // markFilteringSet
        0xde, 0xad, 0xbe, 0xef, // opaque payload bytes
    ];
    let lookup = Lookup::decode(
        &bytes,
        ByteSpan::between(2, bytes.len()).unwrap(),
        LayoutTableKind::Positioning,
    )
    .unwrap();

    assert_eq!(lookup.lookup_type(), 3);
    assert!(lookup.flags().right_to_left());
    assert!(lookup.flags().ignores_base_glyphs());
    assert!(lookup.flags().ignores_ligatures());
    assert_eq!(lookup.flags().mark_filter(), MarkFilter::Set(5));
    assert_eq!(
        lookup.subtables(),
        [
            ByteSpan::between(14, 18).unwrap(),
            ByteSpan::between(16, 18).unwrap()
        ]
    );
}

#[test]
fn ignore_marks_dominates_declared_mark_filters() {
    let bytes = [
        0x00, 0x01, 0x07, 0x18, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x03, 0xfe,
    ];
    let lookup = Lookup::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap();

    assert_eq!(lookup.flags().mark_filter(), MarkFilter::IgnoreAll);
}

#[test]
fn decodes_mark_attachment_class_without_filtering_set() {
    let bytes = [0x00, 0x01, 0x0b, 0x00, 0x00, 0x01, 0x00, 0x08, 0xfe];
    let lookup = Lookup::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap();

    assert_eq!(
        lookup.flags().mark_filter(),
        MarkFilter::AttachmentClass(11)
    );
}

#[test]
fn rejects_lookup_type_outside_table_enumeration() {
    let bytes = [0xaa, 0xbb, 0x00, 0x09];
    let diagnostic = Lookup::decode(
        &bytes,
        ByteSpan::between(2, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(2, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLookupType {
            table: LayoutTableKind::Substitution,
            lookup_type: 9,
        })
    );
}

#[test]
fn rejects_reserved_lookup_flag_bits() {
    let bytes = [0x00, 0x01, 0x00, 0x20];
    let diagnostic = Lookup::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(2, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::ReservedLookupFlags { flags: 0x0020 })
    );
}

#[test]
fn rejects_lookup_without_subtables() {
    let bytes = [0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
    let diagnostic = Lookup::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Positioning,
    )
    .unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(4, 6));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::EmptyLookupSubtables {
            table: LayoutTableKind::Positioning,
            lookup_type: 1,
        })
    );
}

#[test]
fn rejects_subtable_target_inside_lookup_header() {
    let bytes = [0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x06, 0xfe];
    let diagnostic = Lookup::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(6, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::LookupSubtable,
            offset: 6,
            minimum: 8,
            container_length: 9,
        })
    );
}

#[test]
fn filtering_set_extends_the_protected_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x10, 0x00, 0x01, 0x00, 0x08, 0x00, 0x03, 0xfe,
    ];
    let diagnostic = Lookup::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::LookupSubtable,
            offset: 8,
            minimum: 10,
            container_length: 11,
        })
    );
}

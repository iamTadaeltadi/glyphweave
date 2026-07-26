//! Integration coverage for family-aware semantic GSUB Lookup decoding.

use glyphweave_layout::{
    GsubLookup, GsubLookupList, GsubSubtable, Lookup, LookupFlags, LookupList,
};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTableKind};

#[test]
fn decodes_same_typed_subtables_in_lookup_order() {
    let bytes = [
        0x00, 0x01, 0x00, 0x06, 0x00, 0x01, // SingleSubst delta +1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
        0x00, 0x01, 0x00, 0x06, 0x00, 0x02, // SingleSubst delta +2
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // Coverage {7}
    ];
    let lookup = Lookup::new(
        LayoutTableKind::Substitution,
        1,
        LookupFlags::default(),
        [
            ByteSpan::between(0, 12).unwrap(),
            ByteSpan::between(12, bytes.len()).unwrap(),
        ],
    );
    let decoded = GsubLookup::decode(&bytes, &lookup).unwrap();

    assert_eq!(decoded.lookup_type(), 1);
    assert_eq!(decoded.len(), 2);
    let GsubSubtable::Single(first) = &decoded.subtables()[0] else {
        panic!("lookup type 1 must dispatch to SingleSubst");
    };
    let GsubSubtable::Single(second) = &decoded.subtables()[1] else {
        panic!("lookup type 1 must dispatch to SingleSubst");
    };
    assert_eq!(first.replacement(GlyphId::new(3)), Some(GlyphId::new(4)));
    assert_eq!(second.replacement(GlyphId::new(7)), Some(GlyphId::new(9)));
}

#[test]
fn rejects_positioning_lookup_before_subtable_dispatch() {
    let lookup = Lookup::new(
        LayoutTableKind::Positioning,
        1,
        LookupFlags::default(),
        [ByteSpan::between(0, 2).unwrap()],
    );
    let diagnostic = GsubLookup::decode(&[0, 0], &lookup).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnexpectedLayoutTableKind {
            expected: LayoutTableKind::Substitution,
            actual: LayoutTableKind::Positioning,
        })
    );
}

#[test]
fn semantic_lookup_list_preserves_mixed_lookup_indices() {
    let bytes = [
        0x00, 0x01, 0x00, 0x06, 0x00, 0x01, // SingleSubst
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
        0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x08, // AlternateSubst
        0x00, 0x01, 0x00, 0x09, // AlternateSet {9}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // Coverage {7}
    ];
    let list = LookupList::new(
        LayoutTableKind::Substitution,
        [
            Lookup::new(
                LayoutTableKind::Substitution,
                1,
                LookupFlags::default(),
                [ByteSpan::between(0, 12).unwrap()],
            ),
            Lookup::new(
                LayoutTableKind::Substitution,
                3,
                LookupFlags::default(),
                [ByteSpan::between(12, bytes.len()).unwrap()],
            ),
        ],
    );
    let decoded = GsubLookupList::decode(&bytes, &list).unwrap();

    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded.lookup(0).unwrap().lookup_type(), 1);
    assert_eq!(decoded.lookup(1).unwrap().lookup_type(), 3);
    assert_eq!(decoded.lookup(2), None);
}

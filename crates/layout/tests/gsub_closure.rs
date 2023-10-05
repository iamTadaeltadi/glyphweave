//! Integration coverage for GSUB glyph and Lookup fixed points.

use glyphweave_layout::{GsubClosure, GsubLookupList, Lookup, LookupFlags, LookupList};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTableKind, Stage};

#[test]
fn closes_transitive_single_multiple_and_ligature_outputs() {
    let bytes = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, // Single 3->4
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x08, // Multiple
        0x00, 0x02, 0x00, 0x05, 0x00, 0x06, // sequence [5,6]
        0x00, 0x01, 0x00, 0x01, 0x00, 0x04, // Coverage {4}
        0x00, 0x01, 0x00, 0x12, 0x00, 0x01, 0x00, 0x08, // Ligature
        0x00, 0x01, 0x00, 0x04, // LigatureSet
        0x00, 0x07, 0x00, 0x02, 0x00, 0x06, // 5,6->7
        0x00, 0x01, 0x00, 0x01, 0x00, 0x05, // Coverage {5}
    ];
    let list = LookupList::new(
        LayoutTableKind::Substitution,
        [
            lookup(1, ByteSpan::between(0, 14).unwrap()),
            lookup(2, ByteSpan::between(14, 34).unwrap()),
            lookup(4, ByteSpan::between(34, bytes.len()).unwrap()),
        ],
    );
    let semantic = GsubLookupList::decode(&bytes, &list).unwrap();
    let closure = GsubClosure::compute(&semantic, [GlyphId::new(3)], [0, 1, 2]).unwrap();

    assert_eq!(
        closure.glyphs().collect::<Vec<_>>(),
        [
            GlyphId::new(3),
            GlyphId::new(4),
            GlyphId::new(5),
            GlyphId::new(6),
            GlyphId::new(7),
        ]
    );
    assert_eq!(closure.lookup_indices().collect::<Vec<_>>(), [0, 1, 2]);
    assert_eq!(closure.passes(), 1);
}

#[test]
fn contextual_action_reaches_nested_lookup_before_its_output() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c, // Context format 3
        0x00, 0x00, 0x00, 0x01, // input 0 invokes Lookup 1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // input Coverage {3}
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, // Single 3->4
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];
    let list = LookupList::new(
        LayoutTableKind::Substitution,
        [
            lookup(5, ByteSpan::between(0, 18).unwrap()),
            lookup(1, ByteSpan::between(18, bytes.len()).unwrap()),
        ],
    );
    let semantic = GsubLookupList::decode(&bytes, &list).unwrap();
    let closure = GsubClosure::compute(&semantic, [GlyphId::new(3)], [0]).unwrap();

    assert_eq!(closure.lookup_indices().collect::<Vec<_>>(), [0, 1]);
    assert_eq!(
        closure.glyphs().collect::<Vec<_>>(),
        [GlyphId::new(3), GlyphId::new(4),]
    );
    assert_eq!(closure.passes(), 2);
}

#[test]
fn cyclic_contextual_lookup_graph_reaches_a_fixed_point() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c, // first Context
        0x00, 0x00, 0x00, 0x01, // Lookup 0 invokes Lookup 1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c, // second Context
        0x00, 0x00, 0x00, 0x00, // Lookup 1 invokes Lookup 0
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];
    let list = LookupList::new(
        LayoutTableKind::Substitution,
        [
            lookup(5, ByteSpan::between(0, 18).unwrap()),
            lookup(5, ByteSpan::between(18, bytes.len()).unwrap()),
        ],
    );
    let semantic = GsubLookupList::decode(&bytes, &list).unwrap();
    let closure = GsubClosure::compute(&semantic, [GlyphId::new(3)], [0]).unwrap();

    assert_eq!(closure.lookup_indices().collect::<Vec<_>>(), [0, 1]);
    assert_eq!(closure.glyphs().collect::<Vec<_>>(), [GlyphId::new(3)]);
    assert_eq!(closure.passes(), 1);
}

#[test]
fn rejects_dangling_contextual_lookup_at_closure_stage() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x02, // dangling Lookup index 2
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let list = LookupList::new(
        LayoutTableKind::Substitution,
        [lookup(5, ByteSpan::between(0, bytes.len()).unwrap())],
    );
    let semantic = GsubLookupList::decode(&bytes, &list).unwrap();
    let diagnostic = GsubClosure::compute(&semantic, [GlyphId::new(3)], [0]).unwrap_err();

    assert_eq!(diagnostic.stage(), Stage::Closure);
    assert_eq!(diagnostic.span(), None);
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidContextLookupIndex {
            lookup_index: 2,
            lookup_count: 1,
        })
    );
}

fn lookup(lookup_type: u16, span: ByteSpan) -> Lookup {
    Lookup::new(
        LayoutTableKind::Substitution,
        lookup_type,
        LookupFlags::default(),
        [span],
    )
}

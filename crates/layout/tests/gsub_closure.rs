//! Integration coverage for GSUB glyph and Lookup fixed points.

use glyphweave_layout::{GsubClosure, GsubLookupList, Lookup, LookupFlags, LookupList};
use glyphweave_sfnt::{ByteSpan, GlyphId, LayoutTableKind};

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

fn lookup(lookup_type: u16, span: ByteSpan) -> Lookup {
    Lookup::new(
        LayoutTableKind::Substitution,
        lookup_type,
        LookupFlags::default(),
        [span],
    )
}

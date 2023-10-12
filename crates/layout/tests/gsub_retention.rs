//! Integration coverage for glyph-closed GSUB retention.

use glyphweave_layout::{
    GsubClosure, GsubLookupList, GsubSubtable, Lookup, LookupFlags, LookupList,
};
use glyphweave_sfnt::{ByteSpan, GlyphId, LayoutTableKind};

#[test]
fn prunes_direct_mappings_outside_the_computed_closure() {
    let bytes = [
        0x00, 0x02, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x04, 0x00, 0x09, // replacements
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x08, // Coverage {3,8}
    ];
    let semantic = decode(&bytes, [(1, ByteSpan::between(0, bytes.len()).unwrap())]);
    let closure = GsubClosure::compute(&semantic, [GlyphId::new(3)], [0]).unwrap();
    let retained = semantic.retain(&closure);

    let GsubSubtable::Single(single) = &retained.lookup(0).unwrap().subtables()[0] else {
        panic!("lookup type 1 must remain SingleSubst");
    };
    assert_eq!(
        single.mappings().collect::<Vec<_>>(),
        [(GlyphId::new(3), GlyphId::new(4))]
    );
}

fn decode<const N: usize>(bytes: &[u8], entries: [(u16, ByteSpan); N]) -> GsubLookupList {
    let list = LookupList::new(
        LayoutTableKind::Substitution,
        entries.map(|(lookup_type, span)| {
            Lookup::new(
                LayoutTableKind::Substitution,
                lookup_type,
                LookupFlags::default(),
                [span],
            )
        }),
    );
    GsubLookupList::decode(bytes, &list).unwrap()
}

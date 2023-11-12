//! Cross-pass integration between GSUB retention and the canonical outline Remap.

use glyphweave_layout::{
    GsubClosure, GsubLookupList, GsubSubtable, Lookup, LookupFlags, LookupList,
};
use glyphweave_outline::{GlyphSet, Remap};
use glyphweave_sfnt::{ByteSpan, GlyphId, LayoutTableKind, NewGlyphId};

#[test]
fn retained_gsub_consumes_the_canonical_dense_remap() {
    let bytes = [
        0x00, 0x01, 0x00, 0x06, 0x00, 0x01, // Single delta +1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];
    let lookup = Lookup::new(
        LayoutTableKind::Substitution,
        1,
        LookupFlags::default(),
        [ByteSpan::between(0, bytes.len()).unwrap()],
    );
    let list = LookupList::new(LayoutTableKind::Substitution, [lookup]);
    let semantic = GsubLookupList::decode(&bytes, &list).unwrap();
    let closure = GsubClosure::compute(&semantic, [GlyphId::new(3)], [0]).unwrap();
    let glyphs = closure.glyphs().collect::<GlyphSet>();
    let remap = Remap::new(&glyphs);

    let emitted = semantic
        .retain(&closure)
        .remap(|glyph| remap.new_glyph_id(glyph))
        .unwrap();

    let GsubSubtable::Single(single) = &emitted.lookup(0).unwrap().subtables()[0] else {
        panic!("lookup type 1 must remain SingleSubst");
    };
    assert_eq!(
        single.mappings().collect::<Vec<_>>(),
        [(NewGlyphId::new(1), NewGlyphId::new(2))]
    );
}

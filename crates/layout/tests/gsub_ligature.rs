//! Integration coverage for semantic LigatureSubst decoding.

use glyphweave_layout::LigatureSubstitution;
use glyphweave_sfnt::{ByteSpan, GlyphId};

#[test]
fn decodes_coverage_sets_and_ligature_preference_order() {
    let bytes = [
        0x00, 0x01, 0x00, 0x28, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x1e, // header
        0x00, 0x02, 0x00, 0x06, 0x00, 0x0e, // first LigatureSet
        0x00, 0x14, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, // 3,4,5 -> 20
        0x00, 0x15, 0x00, 0x02, 0x00, 0x06, // 3,6 -> 21
        0x00, 0x01, 0x00, 0x04, // second LigatureSet
        0x00, 0x16, 0x00, 0x02, 0x00, 0x08, // 7,8 -> 22
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // Coverage
    ];
    let substitution =
        LigatureSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(substitution.len(), 3);
    assert_eq!(
        substitution.rules()[0].components(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5),]
    );
    assert_eq!(substitution.rules()[0].replacement(), GlyphId::new(20));
    assert_eq!(
        substitution.rules()[1].components(),
        [GlyphId::new(3), GlyphId::new(6),]
    );
    assert_eq!(
        substitution.rules()[2].components(),
        [GlyphId::new(7), GlyphId::new(8),]
    );
}

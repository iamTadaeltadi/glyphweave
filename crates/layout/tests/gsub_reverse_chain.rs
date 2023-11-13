//! Integration coverage for `ReverseChainSingleSubst` decoding.

use glyphweave_layout::ReverseChainSingleSubstitution;
use glyphweave_sfnt::{ByteSpan, GlyphId};

#[test]
fn decodes_reverse_context_and_parallel_substitutes() {
    let bytes = [
        0x00, 0x01, 0x00, 0x26, // format and input Coverage
        0x00, 0x02, 0x00, 0x14, 0x00, 0x1a, // two backtrack Coverages
        0x00, 0x01, 0x00, 0x20, // one lookahead Coverage
        0x00, 0x02, 0x00, 0x1e, 0x00, 0x28, // outputs 30 and 40
        0x00, 0x01, 0x00, 0x01, 0x00, 0x01, // nearest backtrack {1}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, // farthest backtrack {2}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x09, // lookahead {9}
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, // input {3, 4}
    ];
    let substitution =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap();

    assert_eq!(substitution.backtrack()[0].glyphs(), [GlyphId::new(1)]);
    assert_eq!(substitution.backtrack()[1].glyphs(), [GlyphId::new(2)]);
    assert_eq!(substitution.lookahead()[0].glyphs(), [GlyphId::new(9)]);
    assert_eq!(
        substitution.substitutions().replacement(GlyphId::new(3)),
        Some(GlyphId::new(30))
    );
    assert_eq!(
        substitution.substitutions().replacement(GlyphId::new(4)),
        Some(GlyphId::new(40))
    );
}

//! Integration coverage for semantic SingleSubst decoding.

use glyphweave_layout::SingleSubstitution;
use glyphweave_sfnt::{ByteSpan, GlyphId};

#[test]
fn decodes_wrapping_single_delta_substitution() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x01, // format 1
        0x00, 0x06, // coverageOffset
        0xff, 0xfe, // deltaGlyphID = -2
        0x00, 0x01, // Coverage format 1
        0x00, 0x02, // glyphCount
        0x00, 0x01, 0xff, 0xff, // covered glyphs
    ];
    let substitution =
        SingleSubstitution::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        substitution.replacement(GlyphId::new(1)),
        Some(GlyphId::new(65_535))
    );
    assert_eq!(
        substitution.replacement(GlyphId::new(65_535)),
        Some(GlyphId::new(65_533))
    );
}

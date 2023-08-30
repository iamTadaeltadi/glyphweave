//! Integration coverage for semantic AlternateSubst decoding.

use glyphweave_layout::AlternateSubstitution;
use glyphweave_sfnt::{ByteSpan, GlyphId};

#[test]
fn decodes_alternate_sets_in_preference_order() {
    let bytes = [
        0x00, 0x01, // format 1
        0x00, 0x14, // coverageOffset
        0x00, 0x02, // alternateSetCount
        0x00, 0x0a, 0x00, 0x10, // alternateSetOffsets
        0x00, 0x02, 0x00, 0x1e, 0x00, 0x14, // alternates [30, 20]
        0x00, 0x01, 0x00, 0x0c, // alternate [12]
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // Coverage
    ];
    let substitution =
        AlternateSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        substitution.alternates(GlyphId::new(3)),
        Some(&[GlyphId::new(30), GlyphId::new(20),][..])
    );
    assert_eq!(
        substitution.alternates(GlyphId::new(7)),
        Some(&[GlyphId::new(12),][..])
    );
}

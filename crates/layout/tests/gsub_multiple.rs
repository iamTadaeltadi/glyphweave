//! Integration coverage for semantic MultipleSubst decoding.

use glyphweave_layout::MultipleSubstitution;
use glyphweave_sfnt::{ByteSpan, GlyphId};

#[test]
fn decodes_ordered_sequences_and_explicit_deletion() {
    let bytes = [
        0x00, 0x01, // format 1
        0x00, 0x12, // coverageOffset
        0x00, 0x02, // sequenceCount
        0x00, 0x0a, 0x00, 0x10, // sequenceOffsets
        0x00, 0x02, 0x00, 0x14, 0x00, 0x15, // sequence [20, 21]
        0x00, 0x00, // empty deletion sequence
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // Coverage
    ];
    let substitution =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        substitution.replacement(GlyphId::new(3)),
        Some(&[GlyphId::new(20), GlyphId::new(21),][..])
    );
    assert_eq!(substitution.replacement(GlyphId::new(7)), Some(&[][..]));
}

#[test]
fn accepts_exact_sequence_target_aliasing() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x02, // format, Coverage, count
        0x00, 0x0a, 0x00, 0x0a, // equal Sequence offsets
        0x00, 0x01, 0x00, 0x08, // shared one-glyph Sequence
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // Coverage
    ];
    let substitution =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        substitution.replacement(GlyphId::new(3)),
        Some(&[GlyphId::new(8),][..])
    );
    assert_eq!(
        substitution.replacement(GlyphId::new(3)),
        substitution.replacement(GlyphId::new(7))
    );
}

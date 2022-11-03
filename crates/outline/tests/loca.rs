//! Semantic coverage for canonical TrueType glyph locations.

use glyphweave_outline::Loca;
use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};

#[test]
fn short_entries_expand_half_offsets_into_absolute_glyf_spans() {
    let mut bytes = vec![0, 0, 0, 2, 0, 5];
    bytes.extend_from_slice(&[0xaa; 10]);
    let table_span = ByteSpan::sized(0, 6).unwrap();
    let glyf_span = ByteSpan::sized(6, 10).unwrap();

    let decoded = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Short)
        .expect("valid short offsets");

    assert_eq!(decoded.offsets(), [0, 4, 10]);
    assert_eq!(
        decoded.glyph_span(GlyphId::NOTDEF),
        ByteSpan::between(6, 10)
    );
    assert_eq!(
        decoded.glyph_span(GlyphId::new(1)),
        ByteSpan::between(10, 16)
    );
}

#[test]
fn long_entries_preserve_full_offsets_at_an_absolute_table_position() {
    let mut bytes = vec![0xaa; 5];
    for offset in [0_u32, 4, 10] {
        bytes.extend_from_slice(&offset.to_be_bytes());
    }
    bytes.extend_from_slice(&[0xbb; 10]);
    let table_span = ByteSpan::sized(5, 12).unwrap();
    let glyf_span = ByteSpan::sized(17, 10).unwrap();

    let decoded = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Long)
        .expect("valid long offsets");

    assert_eq!(decoded.offsets(), [0, 4, 10]);
    assert_eq!(
        decoded.glyph_span(GlyphId::NOTDEF),
        ByteSpan::between(17, 21)
    );
    assert_eq!(
        decoded.glyph_span(GlyphId::new(1)),
        ByteSpan::between(21, 27)
    );
}

#[test]
fn repeated_offsets_preserve_empty_glyphs_on_both_sides_of_data() {
    let mut bytes = vec![0, 0, 0, 0, 0, 2, 0, 2];
    bytes.extend_from_slice(&[0xaa; 4]);
    let table_span = ByteSpan::sized(0, 8).unwrap();
    let glyf_span = ByteSpan::sized(8, 4).unwrap();

    let decoded = Loca::decode(&bytes, table_span, glyf_span, 3, LocaFormat::Short)
        .expect("equal consecutive locations are valid");

    assert_eq!(
        decoded.glyph_span(GlyphId::NOTDEF),
        Some(ByteSpan::empty_at(8))
    );
    assert_eq!(
        decoded.glyph_span(GlyphId::new(1)),
        ByteSpan::between(8, 12)
    );
    assert_eq!(
        decoded.glyph_span(GlyphId::new(2)),
        Some(ByteSpan::empty_at(12))
    );
}

#[test]
fn zero_glyph_input_still_requires_and_preserves_one_sentinel() {
    let bytes = [0_u8; 4];
    let table_span = ByteSpan::sized(0, 4).unwrap();
    let glyf_span = ByteSpan::empty_at(4);

    let decoded = Loca::decode(&bytes, table_span, glyf_span, 0, LocaFormat::Long)
        .expect("loca relation is structurally empty");

    assert_eq!(decoded.glyph_count(), 0);
    assert_eq!(decoded.offsets(), [0]);
    assert_eq!(decoded.glyph_span(GlyphId::NOTDEF), None);
}

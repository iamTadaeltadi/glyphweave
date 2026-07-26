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

#[test]
fn short_format_reaches_its_maximal_even_glyf_offset() {
    let mut bytes = vec![0, 0, 0xff, 0xff];
    let glyf_length = usize::from(u16::MAX) * 2;
    bytes.resize(4 + glyf_length, 0xaa);
    let table_span = ByteSpan::sized(0, 4).unwrap();
    let glyf_span = ByteSpan::sized(4, glyf_length).unwrap();

    let decoded = Loca::decode(&bytes, table_span, glyf_span, 1, LocaFormat::Short)
        .expect("maximum short offset is representable");

    assert_eq!(decoded.offsets(), [0, 131_070]);
    assert_eq!(
        decoded.glyph_span(GlyphId::NOTDEF).map(ByteSpan::length),
        Some(glyf_length)
    );
}

#[test]
fn maximal_declared_glyph_count_keeps_its_extra_sentinel() {
    let glyph_count = u16::MAX;
    let table_length = (usize::from(glyph_count) + 1) * 2;
    let bytes = vec![0_u8; table_length];
    let table_span = ByteSpan::sized(0, table_length).unwrap();
    let glyf_span = ByteSpan::empty_at(table_length);

    let decoded = Loca::decode(
        &bytes,
        table_span,
        glyf_span,
        glyph_count,
        LocaFormat::Short,
    )
    .expect("the complete maxp glyph-count domain is representable");

    assert_eq!(decoded.offsets().len(), usize::from(glyph_count) + 1);
    assert_eq!(
        decoded.glyph_span(GlyphId::new(glyph_count - 1)),
        Some(glyf_span)
    );
    assert_eq!(decoded.glyph_span(GlyphId::new(glyph_count)), None);
}

#[test]
fn glyf_may_precede_loca_in_the_physical_font_layout() {
    let mut bytes = vec![0xaa; 6];
    bytes.extend_from_slice(&[0, 0, 0, 1, 0, 3]);
    let glyf_span = ByteSpan::sized(0, 6).unwrap();
    let table_span = ByteSpan::sized(6, 6).unwrap();

    let decoded = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Short)
        .expect("table directory controls physical order");

    assert_eq!(decoded.glyph_span(GlyphId::NOTDEF), ByteSpan::between(0, 2));
    assert_eq!(decoded.glyph_span(GlyphId::new(1)), ByteSpan::between(2, 6));
}

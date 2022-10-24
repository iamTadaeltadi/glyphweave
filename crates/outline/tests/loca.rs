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

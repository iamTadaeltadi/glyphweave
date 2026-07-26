//! Cross-table integration from `loca` entries into composite decoding.

use glyphweave_outline::{CompositeGlyph, Loca};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LocaFormat};

#[test]
fn decoded_loca_span_feeds_the_composite_decoder_without_rebasing() {
    let mut bytes = vec![0, 0, 0, 8];
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x00, 0x02, 0x00, 0x2a, 0x00, 0x00]);
    let loca_span = ByteSpan::sized(0, 4).unwrap();
    let glyf_span = ByteSpan::sized(4, 16).unwrap();
    let loca = Loca::decode(&bytes, loca_span, glyf_span, 1, LocaFormat::Short).unwrap();

    let glyph_span = loca.glyph_span(GlyphId::NOTDEF).unwrap();
    let glyph = CompositeGlyph::decode(&bytes, glyph_span).unwrap();

    assert_eq!(glyph.span(), glyf_span);
    assert_eq!(glyph.components()[0].glyph_id(), GlyphId::new(42));
}

#[test]
fn component_records_cannot_cross_a_loca_boundary() {
    let mut bytes = vec![0, 0, 0, 8, 0, 16];
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x00, 0x22, 0x00, 0x2a, 0x00, 0x00]);
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x00, 0x02, 0x00, 0x2b, 0x00, 0x00]);
    let loca_span = ByteSpan::sized(0, 6).unwrap();
    let glyf_span = ByteSpan::sized(6, 32).unwrap();
    let loca = Loca::decode(&bytes, loca_span, glyf_span, 2, LocaFormat::Short).unwrap();

    let first_span = loca.glyph_span(GlyphId::NOTDEF).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, first_span)
        .expect_err("the second glyph cannot satisfy MORE_COMPONENTS in the first");

    assert_eq!(diagnostic.span(), Some(ByteSpan::empty_at(22)));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 0,
        }
    );
}

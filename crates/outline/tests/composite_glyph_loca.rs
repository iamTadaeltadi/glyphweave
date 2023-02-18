//! Cross-table integration from `loca` entries into composite decoding.

use glyphweave_outline::{CompositeGlyph, Loca};
use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};

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

//! Cross-table integration from `loca` entries into simple-glyph decoding.

use glyphweave_outline::{Loca, SimpleGlyph};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LocaFormat, OutlineCause};

#[test]
fn decoded_loca_span_feeds_the_simple_glyph_decoder_without_rebasing() {
    let mut bytes = vec![0, 0, 0, 5];
    bytes.extend_from_slice(&[0; 10]);
    let loca_span = ByteSpan::sized(0, 4).unwrap();
    let glyf_span = ByteSpan::sized(4, 10).unwrap();
    let loca = Loca::decode(&bytes, loca_span, glyf_span, 1, LocaFormat::Short).unwrap();

    let glyph_span = loca.glyph_span(GlyphId::new(0)).unwrap();
    let glyph = SimpleGlyph::decode(&bytes, glyph_span).unwrap();

    assert_eq!(glyph.span(), glyf_span);
    assert_eq!(glyph.contour_count(), 0);
    assert!(glyph.points().is_empty());
}

#[test]
fn repeated_loca_offsets_remain_glyphs_without_descriptions() {
    let bytes = [0; 4];
    let loca_span = ByteSpan::sized(0, 4).unwrap();
    let glyf_span = ByteSpan::empty_at(4);
    let loca = Loca::decode(&bytes, loca_span, glyf_span, 1, LocaFormat::Short).unwrap();

    let glyph_span = loca.glyph_span(GlyphId::new(0)).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, glyph_span)
        .expect_err("an empty loca range has no Glyph Header");

    assert_eq!(glyph_span, glyf_span);
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::EmptyGlyphDescription)
    );
}

use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};

use super::Loca;

fn model() -> Loca {
    Loca {
        table_span: ByteSpan::sized(20, 6).unwrap(),
        glyf_span: ByteSpan::sized(100, 6).unwrap(),
        format: LocaFormat::Short,
        glyph_count: 2,
        offsets: vec![0, 0, 6],
    }
}

#[test]
fn accessors_preserve_cross_table_identity() {
    let loca = model();

    assert_eq!(loca.table_span(), ByteSpan::sized(20, 6).unwrap());
    assert_eq!(loca.glyf_span(), ByteSpan::sized(100, 6).unwrap());
    assert_eq!(loca.format(), LocaFormat::Short);
    assert_eq!(loca.glyph_count(), 2);
    assert_eq!(loca.offsets(), [0, 0, 6]);
}

#[test]
fn equal_offsets_produce_an_empty_glyph_span() {
    let loca = model();

    assert_eq!(loca.glyph_span(GlyphId::NOTDEF), ByteSpan::sized(100, 0));
    assert_eq!(loca.glyph_span(GlyphId::new(1)), ByteSpan::sized(100, 6));
}

#[test]
fn glyph_lookup_is_undefined_at_and_after_the_count() {
    let loca = model();

    assert_eq!(loca.glyph_span(GlyphId::new(2)), None);
    assert_eq!(loca.glyph_span(GlyphId::new(u16::MAX)), None);
}

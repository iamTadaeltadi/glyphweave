//! Integration coverage for eager `glyf` decoding.

use glyphweave_outline::{Glyf, Loca};
use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};

#[test]
fn equal_loca_offsets_preserve_empty_outlines() {
    let bytes = [0; 12];
    let loca_span = ByteSpan::sized(0, 12).unwrap();
    let glyf_span = ByteSpan::empty_at(12);
    let loca = Loca::decode(&bytes, loca_span, glyf_span, 2, LocaFormat::Long).unwrap();

    let glyf = Glyf::decode(&bytes, &loca).unwrap();

    assert_eq!(glyf.table_span(), glyf_span);
    assert_eq!(glyf.glyph_count(), 2);
    assert_eq!(glyf.outlines().len(), 2);
    assert!(glyf.outline(GlyphId::NOTDEF).unwrap().is_empty());
    assert!(glyf.outline(GlyphId::new(1)).unwrap().is_empty());
    assert_eq!(glyf.outline(GlyphId::new(2)), None);
}

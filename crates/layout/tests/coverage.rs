//! Semantic decoding tests for OpenType Coverage tables.

use glyphweave_layout::Coverage;
use glyphweave_sfnt::GlyphId;

#[test]
fn decodes_format_1_glyphs_and_indices() {
    let bytes = [
        0x00, 0x01, // format
        0x00, 0x03, // glyphCount
        0x00, 0x02, 0x00, 0x08, 0xff, 0xff,
    ];
    let coverage = Coverage::decode(&bytes, 0).unwrap();

    assert_eq!(
        coverage.glyphs(),
        &[GlyphId::new(2), GlyphId::new(8), GlyphId::new(u16::MAX)]
    );
    assert_eq!(coverage.coverage_index(GlyphId::new(8)), Some(1));
}

#[test]
fn decodes_format_1_at_an_absolute_offset() {
    let bytes = [0xaa, 0xbb, 0x00, 0x01, 0x00, 0x01, 0x00, 0x09];
    let coverage = Coverage::decode(&bytes, 2).unwrap();

    assert_eq!(coverage.glyphs(), &[GlyphId::new(9)]);
}

#[test]
fn decodes_empty_format_1() {
    let coverage = Coverage::decode(&[0x00, 0x01, 0x00, 0x00], 0).unwrap();
    assert!(coverage.is_empty());
}

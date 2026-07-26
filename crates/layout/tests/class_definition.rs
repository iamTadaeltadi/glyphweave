//! Semantic decoding tests for OpenType Class Definition tables.

use glyphweave_layout::ClassDefinition;
use glyphweave_sfnt::GlyphId;

#[test]
fn decodes_format_1_and_discards_class_zero() {
    let bytes = [
        0x00, 0x01, // format
        0x00, 0x05, // startGlyphID
        0x00, 0x04, // glyphCount
        0x00, 0x02, 0x00, 0x00, 0x00, 0x07, 0x00, 0x02,
    ];
    let classes = ClassDefinition::decode(&bytes, 0).unwrap();

    assert_eq!(classes.class_of(GlyphId::new(5)), 2);
    assert_eq!(classes.class_of(GlyphId::new(6)), 0);
    assert_eq!(classes.class_of(GlyphId::new(7)), 7);
    assert_eq!(classes.class_of(GlyphId::new(8)), 2);
    assert_eq!(classes.assignments().len(), 3);
}

#[test]
fn decodes_format_1_at_an_absolute_offset() {
    let bytes = [0xaa, 0xbb, 0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00, 0x04];
    let classes = ClassDefinition::decode(&bytes, 2).unwrap();

    assert_eq!(classes.class_of(GlyphId::new(9)), 4);
}

#[test]
fn decodes_empty_format_1_at_maximum_start() {
    let bytes = [0x00, 0x01, 0xff, 0xff, 0x00, 0x00];
    let classes = ClassDefinition::decode(&bytes, 0).unwrap();
    assert!(classes.is_empty());
}

#[test]
fn decodes_single_class_at_maximum_glyph() {
    let bytes = [0x00, 0x01, 0xff, 0xff, 0x00, 0x01, 0x00, 0x03];
    let classes = ClassDefinition::decode(&bytes, 0).unwrap();
    assert_eq!(classes.class_of(GlyphId::new(u16::MAX)), 3);
}

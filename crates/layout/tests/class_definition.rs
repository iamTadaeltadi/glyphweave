//! Semantic decoding tests for OpenType Class Definition tables.

use glyphweave_layout::ClassDefinition;
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause};

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

#[test]
fn rejects_unknown_class_definition_format() {
    let diagnostic = ClassDefinition::decode(&[0xaa, 0x00, 0x03], 1).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(1, 3));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedClassDefinitionFormat { format: 3 })
    );
}

#[test]
fn rejects_format_1_range_beyond_maximum_glyph() {
    let bytes = [0x00, 0x01, 0xff, 0xfe, 0x00, 0x03];
    let diagnostic = ClassDefinition::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(2, 6));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::ClassDefinitionGlyphRangeOverflow {
            start: GlyphId::new(65_534),
            count: 3,
        })
    );
}

#[test]
fn decodes_format_2_ranges_and_discards_class_zero() {
    let bytes = [
        0x00, 0x02, 0x00, 0x03, // format, classRangeCount
        0x00, 0x02, 0x00, 0x04, 0x00, 0x03, // 2..=4 => class 3
        0x00, 0x05, 0x00, 0x06, 0x00, 0x00, // adjacent class zero
        0x00, 0x09, 0x00, 0x09, 0x00, 0x07, // 9 => class 7
    ];
    let classes = ClassDefinition::decode(&bytes, 0).unwrap();

    assert_eq!(classes.class_of(GlyphId::new(2)), 3);
    assert_eq!(classes.class_of(GlyphId::new(4)), 3);
    assert_eq!(classes.class_of(GlyphId::new(5)), 0);
    assert_eq!(classes.class_of(GlyphId::new(9)), 7);
    assert_eq!(classes.len(), 4);
}

#[test]
fn decodes_empty_format_2() {
    let classes = ClassDefinition::decode(&[0x00, 0x02, 0x00, 0x00], 0).unwrap();
    assert!(classes.is_empty());
}

#[test]
fn canonical_encoding_merges_adjacent_equal_class_ranges() {
    let bytes = [
        0x00, 0x02, 0x00, 0x02, // format, classRangeCount
        0x00, 0x03, 0x00, 0x04, 0x00, 0x06, 0x00, 0x05, 0x00, 0x06, 0x00, 0x06,
    ];
    let classes = ClassDefinition::decode(&bytes, 0).unwrap();

    assert_eq!(
        classes.encode(),
        [0x00, 0x02, 0x00, 0x01, 0x00, 0x03, 0x00, 0x06, 0x00, 0x06]
    );
}

#[test]
fn decodes_maximal_format_2_range() {
    let bytes = [
        0x00, 0x02, 0x00, 0x01, // format, classRangeCount
        0x00, 0x00, 0xff, 0xff, 0x00, 0x01,
    ];
    let classes = ClassDefinition::decode(&bytes, 0).unwrap();

    assert_eq!(classes.len(), 65_536);
    assert_eq!(classes.class_of(GlyphId::new(u16::MAX)), 1);
}

#[test]
fn rejects_reversed_class_range() {
    let bytes = [
        0x00, 0x02, 0x00, 0x01, // format, count
        0x00, 0x08, 0x00, 0x07, 0x00, 0x01,
    ];
    let diagnostic = ClassDefinition::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(4, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidClassDefinitionRange {
            start: GlyphId::new(8),
            end: GlyphId::new(7),
        })
    );
}

#[test]
fn rejects_overlapping_class_ranges() {
    let bytes = [
        0x00, 0x02, 0x00, 0x02, // format, count
        0x00, 0x02, 0x00, 0x05, 0x00, 0x01, 0x00, 0x04, 0x00, 0x08, 0x00, 0x02,
    ];
    let diagnostic = ClassDefinition::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(10, 12));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::OverlappingClassDefinitionRange {
            previous_end: GlyphId::new(5),
            start: GlyphId::new(4),
        })
    );
}

#[test]
fn encodes_sparse_classes_as_a_dense_array() {
    let classes = ClassDefinition::from_assignments([(GlyphId::new(2), 1), (GlyphId::new(4), 7)]);
    assert_eq!(
        classes.encode(),
        [
            0x00, 0x01, // format
            0x00, 0x02, // startGlyphID
            0x00, 0x03, // glyphCount
            0x00, 0x01, 0x00, 0x00, 0x00, 0x07,
        ]
    );
}

#[test]
fn class_format_ties_prefer_format_1() {
    let classes = ClassDefinition::from_assignments([(GlyphId::new(5), 3), (GlyphId::new(6), 3)]);
    assert_eq!(
        classes.encode(),
        [0x00, 0x01, 0x00, 0x05, 0x00, 0x02, 0x00, 0x03, 0x00, 0x03]
    );
}

#[test]
fn empty_class_definition_encodes_as_format_2() {
    assert_eq!(
        ClassDefinition::default().encode(),
        [0x00, 0x02, 0x00, 0x00]
    );
}

#[test]
fn encodes_long_class_runs_as_format_2() {
    let classes = ClassDefinition::from_assignments(
        (3..=5)
            .map(|raw| (GlyphId::new(raw), 1))
            .chain((6..=8).map(|raw| (GlyphId::new(raw), 2))),
    );
    assert_eq!(
        classes.encode(),
        [
            0x00, 0x02, 0x00, 0x02, // format, classRangeCount
            0x00, 0x03, 0x00, 0x05, 0x00, 0x01, // 3..=5 => class 1
            0x00, 0x06, 0x00, 0x08, 0x00, 0x02, // 6..=8 => class 2
        ]
    );
}

#[test]
fn maximal_class_domain_encodes_as_one_range() {
    let classes =
        ClassDefinition::from_assignments((0..=u16::MAX).map(|raw| (GlyphId::new(raw), 9)));
    assert_eq!(
        classes.encode(),
        [0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0xff, 0xff, 0x00, 0x09]
    );
}

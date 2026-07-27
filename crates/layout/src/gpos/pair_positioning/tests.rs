use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::{PairAdjustment, PairPositioning, PairValue};
use crate::ValueRecord;

const fn span(bytes: &[u8]) -> ByteSpan {
    ByteSpan::between(0, bytes.len()).expect("nonempty subtable span")
}

#[test]
fn pair_adjustment_reports_each_record_and_emptiness() {
    let adjustment = PairAdjustment::new(ValueRecord::new(0, 0, -40, 0), ValueRecord::default());
    assert_eq!(adjustment.first().x_advance(), -40);
    assert!(adjustment.second().is_zero());
    assert!(!adjustment.is_zero());
    assert!(PairAdjustment::default().is_zero());
}

#[test]
fn pair_value_binds_a_trailing_glyph_to_its_adjustment() {
    let adjustment = PairAdjustment::new(ValueRecord::new(0, 0, -15, 0), ValueRecord::default());
    let value = PairValue::new(GlyphId::new(7), adjustment);
    assert_eq!(value.second(), GlyphId::new(7));
    assert_eq!(value.adjustment(), adjustment);
}

#[test]
fn decodes_format_1_pair_sets() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x12, // coverageOffset = 18
        0x00, 0x04, // valueFormat1 = X_ADVANCE
        0x00, 0x00, // valueFormat2 = empty
        0x00, 0x01, // pairSetCount = 1
        0x00, 0x0c, // pairSetOffset[0] = 12
        0x00, 0x01, // pairValueCount = 1
        0x00, 0x07, // secondGlyph = 7
        0xff, 0xe2, // valueRecord1.xAdvance = -30
        0x00, 0x01, // coverageFormat = 1
        0x00, 0x01, // glyphCount = 1
        0x00, 0x04, // glyph = 4
    ];
    let positioning = PairPositioning::decode(&bytes, span(&bytes)).unwrap();

    let PairPositioning::Glyphs { coverage, sets } = &positioning else {
        panic!("format 1 decodes to the explicit-glyph form");
    };
    assert_eq!(coverage.glyphs(), &[GlyphId::new(4)]);
    assert_eq!(sets.len(), 1);
    assert_eq!(sets[0].len(), 1);
    assert_eq!(sets[0][0].second(), GlyphId::new(7));

    let expected = PairAdjustment::new(ValueRecord::new(0, 0, -30, 0), ValueRecord::default());
    assert_eq!(
        positioning.adjustment(GlyphId::new(4), GlyphId::new(7)),
        Some(expected)
    );
    assert_eq!(
        positioning.adjustment(GlyphId::new(4), GlyphId::new(8)),
        None
    );
    assert_eq!(
        positioning.adjustment(GlyphId::new(5), GlyphId::new(7)),
        None
    );
    assert_eq!(positioning.coverage().len(), 1);
}

#[test]
fn skips_device_offsets_inside_a_value_record() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x14, // coverageOffset = 20
        0x00, 0x14, // valueFormat1 = X_ADVANCE | X_PLACEMENT_DEVICE
        0x00, 0x00, // valueFormat2 = empty
        0x00, 0x01, // pairSetCount = 1
        0x00, 0x0c, // pairSetOffset[0] = 12
        0x00, 0x01, // pairValueCount = 1
        0x00, 0x07, // secondGlyph = 7
        0xff, 0xec, // valueRecord1.xAdvance = -20
        0x00, 0x05, // valueRecord1.xPlacementDevice offset (skipped)
        0x00, 0x01, // coverageFormat = 1
        0x00, 0x01, // glyphCount = 1
        0x00, 0x04, // glyph = 4
    ];
    let positioning = PairPositioning::decode(&bytes, span(&bytes)).unwrap();
    let expected = PairAdjustment::new(ValueRecord::new(0, 0, -20, 0), ValueRecord::default());
    assert_eq!(
        positioning.adjustment(GlyphId::new(4), GlyphId::new(7)),
        Some(expected)
    );
}

#[test]
fn decodes_format_2_class_grid() {
    let bytes = [
        0x00, 0x02, // posFormat = 2
        0x00, 0x18, // coverageOffset = 24
        0x00, 0x04, // valueFormat1 = X_ADVANCE
        0x00, 0x00, // valueFormat2 = empty
        0x00, 0x1e, // classDef1Offset = 30
        0x00, 0x26, // classDef2Offset = 38
        0x00, 0x02, // class1Count = 2
        0x00, 0x02, // class2Count = 2
        0x00, 0x00, // grid[0][0].xAdvance = 0
        0x00, 0x00, // grid[0][1].xAdvance = 0
        0x00, 0x00, // grid[1][0].xAdvance = 0
        0xff, 0xe7, // grid[1][1].xAdvance = -25
        0x00, 0x01, // coverageFormat = 1
        0x00, 0x01, // glyphCount = 1
        0x00, 0x04, // glyph = 4
        0x00, 0x01, // classDef1 format = 1
        0x00, 0x04, // startGlyphID = 4
        0x00, 0x01, // glyphCount = 1
        0x00, 0x01, // classValue[0] = 1
        0x00, 0x01, // classDef2 format = 1
        0x00, 0x09, // startGlyphID = 9
        0x00, 0x01, // glyphCount = 1
        0x00, 0x01, // classValue[0] = 1
    ];
    let positioning = PairPositioning::decode(&bytes, span(&bytes)).unwrap();

    let PairPositioning::Classes { grid, .. } = &positioning else {
        panic!("format 2 decodes to the class-grid form");
    };
    assert_eq!(grid.len(), 2);
    assert_eq!(grid[0].len(), 2);

    let expected = PairAdjustment::new(ValueRecord::new(0, 0, -25, 0), ValueRecord::default());
    assert_eq!(
        positioning.adjustment(GlyphId::new(4), GlyphId::new(9)),
        Some(expected)
    );
    // A covered leading glyph with an in-range class pair resolves even to a zero adjustment.
    assert_eq!(
        positioning.adjustment(GlyphId::new(4), GlyphId::new(8)),
        Some(PairAdjustment::default())
    );
    // An uncovered leading glyph is never positioned by format 2.
    assert_eq!(
        positioning.adjustment(GlyphId::new(5), GlyphId::new(9)),
        None
    );
    assert_eq!(positioning.coverage().glyphs(), &[GlyphId::new(4)]);
}

#[test]
fn rejects_an_unsupported_format() {
    let bytes = [0x00, 0x03];
    let diagnostic = PairPositioning::decode(&bytes, span(&bytes)).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposPairFormat { format: 3 })
    );
}

#[test]
fn rejects_reserved_bits_in_the_first_value_format() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x00, // coverageOffset
        0x01, 0x00, // valueFormat1 sets a reserved bit
    ];
    let diagnostic = PairPositioning::decode(&bytes, span(&bytes)).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::ReservedGposPairValueFormat {
            record: 1,
            format: 0x0100,
        })
    );
    assert_eq!(diagnostic.span().unwrap().start(), 4);
}

#[test]
fn rejects_reserved_bits_in_the_second_value_format() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x00, // coverageOffset
        0x00, 0x04, // valueFormat1 = X_ADVANCE
        0x01, 0x00, // valueFormat2 sets a reserved bit
    ];
    let diagnostic = PairPositioning::decode(&bytes, span(&bytes)).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::ReservedGposPairValueFormat {
            record: 2,
            format: 0x0100,
        })
    );
}

#[test]
fn rejects_a_pair_set_count_that_disagrees_with_coverage() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0a, // coverageOffset = 10
        0x00, 0x00, // valueFormat1 = empty
        0x00, 0x00, // valueFormat2 = empty
        0x00, 0x00, // pairSetCount = 0
        0x00, 0x01, // coverageFormat = 1
        0x00, 0x01, // glyphCount = 1
        0x00, 0x04, // glyph = 4
    ];
    let diagnostic = PairPositioning::decode(&bytes, span(&bytes)).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGposPairSetCount { inputs: 1, sets: 0 })
    );
}

#[test]
fn rejects_a_coverage_offset_outside_the_subtable() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0xc8, // coverageOffset = 200 (out of range)
        0x00, 0x00, // valueFormat1 = empty
        0x00, 0x00, // valueFormat2 = empty
        0x00, 0x00, // pairSetCount = 0
    ];
    let diagnostic = PairPositioning::decode(&bytes, span(&bytes)).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::PairPositioningCoverage,
            offset: 200,
            minimum: 10,
            container_length: 10,
        })
    );
}

#[test]
fn truncated_subtable_reports_unexpected_end() {
    let bytes = [0x00, 0x01, 0x00];
    assert!(PairPositioning::decode(&bytes, span(&bytes)).is_err());
}

#[test]
fn new_layout_causes_render_readable_messages() {
    assert_eq!(
        LayoutCause::UnsupportedGposPairFormat { format: 5 }.to_string(),
        "unsupported GPOS PairPos format 5"
    );
    assert_eq!(
        LayoutCause::ReservedGposPairValueFormat {
            record: 2,
            format: 0x0100,
        }
        .to_string(),
        "GPOS PairPos value record 2 format 0x0100 sets reserved bits"
    );
    assert_eq!(
        LayoutCause::MismatchedGposPairSetCount { inputs: 3, sets: 2 }.to_string(),
        "GPOS PairPos has 3 covered glyph(s) but 2 pair set(s)"
    );
}

#[test]
fn new_layout_targets_have_stable_names() {
    assert_eq!(
        LayoutTarget::PairPositioningCoverage.to_string(),
        "PairPos Coverage"
    );
    assert_eq!(LayoutTarget::PairSet.to_string(), "PairSet");
    assert_eq!(
        LayoutTarget::PairPositioningClassDefinition.to_string(),
        "PairPos Class Definition"
    );
}

use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause};

use super::SinglePositioning;
use crate::{ValueFormat, ValueRecord};

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

const fn whole(bytes: &[u8]) -> ByteSpan {
    ByteSpan::sized(0, bytes.len()).expect("subtable span fits the buffer")
}

fn layout_cause(bytes: &[u8]) -> LayoutCause {
    let span = whole(bytes);
    match SinglePositioning::decode(bytes, span)
        .expect_err("decode should fail")
        .cause()
    {
        Cause::Layout(cause) => cause,
        other => panic!("expected a layout cause, found {other:?}"),
    }
}

#[test]
fn format_1_shares_one_record_across_all_covered_glyphs() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // posFormat
    push_u16(&mut bytes, 8); // coverageOffset
    push_u16(&mut bytes, 0x0004); // valueFormat: x advance
    push_i16(&mut bytes, -30); // shared valueRecord.xAdvance
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 2); // glyph count
    push_u16(&mut bytes, 4);
    push_u16(&mut bytes, 9);

    let positioning = SinglePositioning::decode(&bytes, whole(&bytes)).unwrap();
    let shared = ValueRecord::new(0, 0, -30, 0);
    assert_eq!(
        positioning.value_format(),
        ValueFormat::from_bits(0x0004).unwrap()
    );
    assert_eq!(positioning.len(), 2);
    assert!(!positioning.is_empty());
    assert_eq!(positioning.adjustment(GlyphId::new(4)), Some(shared));
    assert_eq!(positioning.adjustment(GlyphId::new(9)), Some(shared));
    assert_eq!(positioning.adjustment(GlyphId::new(5)), None);
    assert_eq!(
        positioning.adjustments().collect::<Vec<_>>(),
        [(GlyphId::new(4), shared), (GlyphId::new(9), shared)]
    );
}

#[test]
fn format_1_consumes_device_offsets_after_the_adjustment() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // posFormat
    push_u16(&mut bytes, 10); // coverageOffset
    push_u16(&mut bytes, 0x0011); // valueFormat: x placement + x placement device
    push_i16(&mut bytes, 25); // valueRecord.xPlacement
    push_u16(&mut bytes, 0); // xPlacementDeviceOffset (skipped)
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 1); // glyph count
    push_u16(&mut bytes, 5);

    let positioning = SinglePositioning::decode(&bytes, whole(&bytes)).unwrap();
    assert!(positioning.value_format().has_any_device());
    assert_eq!(
        positioning.adjustment(GlyphId::new(5)),
        Some(ValueRecord::new(25, 0, 0, 0))
    );
}

#[test]
fn format_2_stores_one_record_per_covered_glyph() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 2); // posFormat
    push_u16(&mut bytes, 12); // coverageOffset
    push_u16(&mut bytes, 0x0004); // valueFormat: x advance
    push_u16(&mut bytes, 2); // valueCount
    push_i16(&mut bytes, -10); // record for coverage index 0
    push_i16(&mut bytes, -20); // record for coverage index 1
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 2); // glyph count
    push_u16(&mut bytes, 3);
    push_u16(&mut bytes, 9);

    let positioning = SinglePositioning::decode(&bytes, whole(&bytes)).unwrap();
    assert_eq!(positioning.len(), 2);
    assert_eq!(
        positioning.adjustment(GlyphId::new(3)),
        Some(ValueRecord::new(0, 0, -10, 0))
    );
    assert_eq!(
        positioning.adjustment(GlyphId::new(9)),
        Some(ValueRecord::new(0, 0, -20, 0))
    );
}

#[test]
fn rejects_an_unknown_subtable_format() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 3);
    assert_eq!(
        layout_cause(&bytes),
        LayoutCause::UnsupportedGposSingleFormat { format: 3 }
    );
}

#[test]
fn rejects_a_reserved_value_format() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // posFormat
    push_u16(&mut bytes, 8); // coverageOffset
    push_u16(&mut bytes, 0x0100); // reserved high-byte bit set
    assert_eq!(
        layout_cause(&bytes),
        LayoutCause::ReservedGposSingleValueFormat { format: 0x0100 }
    );
}

#[test]
fn rejects_a_value_count_that_disagrees_with_coverage() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 2); // posFormat
    push_u16(&mut bytes, 10); // coverageOffset
    push_u16(&mut bytes, 0x0004); // valueFormat
    push_u16(&mut bytes, 1); // valueCount: only one record
    push_i16(&mut bytes, -10);
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 2); // covers two glyphs
    push_u16(&mut bytes, 3);
    push_u16(&mut bytes, 9);
    assert_eq!(
        layout_cause(&bytes),
        LayoutCause::MismatchedGposSingleValueCount {
            inputs: 2,
            records: 1,
        }
    );
}

#[test]
fn rejects_a_truncated_subtable() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // posFormat only; header is truncated
    assert!(SinglePositioning::decode(&bytes, whole(&bytes)).is_err());
}

#[test]
fn model_canonicalizes_and_reports_emptiness() {
    let format = ValueFormat::from_bits(0x0004).unwrap();
    let positioning = SinglePositioning::new(
        format,
        [
            (GlyphId::new(9), ValueRecord::new(0, 0, -5, 0)),
            (GlyphId::new(9), ValueRecord::new(0, 0, -7, 0)),
        ],
    );
    assert_eq!(
        positioning.adjustment(GlyphId::new(9)),
        Some(ValueRecord::new(0, 0, -7, 0))
    );
    assert_eq!(positioning.len(), 1);
    assert!(SinglePositioning::<GlyphId>::default().is_empty());
    assert_eq!(
        SinglePositioning::<GlyphId>::default().value_format(),
        ValueFormat::default()
    );
}

#[test]
fn new_layout_causes_render_their_specifics() {
    assert_eq!(
        LayoutCause::UnsupportedGposSingleFormat { format: 4 }.to_string(),
        "unsupported GPOS SinglePos format 4"
    );
    assert_eq!(
        LayoutCause::ReservedGposSingleValueFormat { format: 0x0100 }.to_string(),
        "GPOS SinglePos value format 0x0100 sets a reserved bit"
    );
    assert_eq!(
        LayoutCause::MismatchedGposSingleValueCount {
            inputs: 2,
            records: 1,
        }
        .to_string(),
        "GPOS SinglePos covers 2 glyph(s) but declares 1 value record(s)"
    );
}

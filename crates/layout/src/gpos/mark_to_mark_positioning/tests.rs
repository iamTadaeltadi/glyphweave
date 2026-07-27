use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::{Mark1Record, Mark2Record, MarkToMarkPositioning};
use crate::{Anchor, Coverage};

fn decode(bytes: &[u8]) -> glyphweave_sfnt::Decoded<MarkToMarkPositioning> {
    MarkToMarkPositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
}

/// A well-formed `MarkMarkPos` format 1 subtable with one mark-1 and one mark-2 glyph.
const fn sample() -> [u8; 46] {
    [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0c, // mark1CoverageOffset = 12
        0x00, 0x12, // mark2CoverageOffset = 18
        0x00, 0x01, // markClassCount = 1
        0x00, 0x18, // mark1ArrayOffset = 24
        0x00, 0x24, // mark2ArrayOffset = 36
        0x00, 0x01, 0x00, 0x01, 0x00, 0x0a, // mark1Coverage: glyph 10
        0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // mark2Coverage: glyph 20
        0x00, 0x01, 0x00, 0x00, 0x00, 0x06, // MarkArray: count 1, class 0, anchorOffset 6
        0x00, 0x01, 0x00, 0x05, 0x00, 0x0f, // Anchor A at 30: (5, 15)
        0x00, 0x01, 0x00, 0x04, // Mark2Array: count 1, anchorOffset 4
        0x00, 0x01, 0x00, 0x1e, 0x00, 0x00, // Anchor B at 40: (30, 0)
    ]
}

#[test]
fn records_expose_class_and_anchor() {
    let mark1 = Mark1Record::new(2, Anchor::coordinate(7, -3));
    assert_eq!(mark1.class(), 2);
    assert_eq!(mark1.anchor(), Anchor::coordinate(7, -3));

    let mark2 = Mark2Record::new([Some(Anchor::coordinate(1, 2)), None]);
    assert_eq!(mark2.len(), 2);
    assert!(!mark2.is_empty());
    assert_eq!(mark2.anchors(), &[Some(Anchor::coordinate(1, 2)), None]);
    assert_eq!(mark2.anchor(0), Some(Anchor::coordinate(1, 2)));
    assert_eq!(mark2.anchor(1), None);
    assert_eq!(mark2.anchor(9), None);
    assert!(Mark2Record::default().is_empty());
}

#[test]
fn model_resolves_records_through_coverage() {
    let positioning = MarkToMarkPositioning::new(
        Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(9)]),
        Coverage::from_glyphs([GlyphId::new(20)]),
        2,
        [
            Mark1Record::new(0, Anchor::coordinate(10, 0)),
            Mark1Record::new(1, Anchor::coordinate(0, 20)),
        ],
        [Mark2Record::new([Some(Anchor::coordinate(5, 5)), None])],
    );
    assert_eq!(positioning.mark_class_count(), 2);
    assert_eq!(positioning.mark1_records().len(), 2);
    assert_eq!(positioning.mark2_records().len(), 1);
    assert_eq!(positioning.mark1_coverage().len(), 2);
    assert_eq!(positioning.mark2_coverage().len(), 1);

    assert_eq!(
        positioning.mark1_record(GlyphId::new(9)).unwrap().class(),
        1
    );
    assert_eq!(
        positioning
            .mark2_record(GlyphId::new(20))
            .unwrap()
            .anchor(0),
        Some(Anchor::coordinate(5, 5))
    );
    assert!(positioning.mark1_record(GlyphId::new(5)).is_none());
    assert!(positioning.mark2_record(GlyphId::new(4)).is_none());
    assert!(
        MarkToMarkPositioning::<GlyphId>::default()
            .mark1_records()
            .is_empty()
    );
}

#[test]
fn decodes_format_1_mark_records() {
    let positioning = decode(&sample()).unwrap();
    assert_eq!(positioning.mark_class_count(), 1);
    assert_eq!(positioning.mark1_coverage().glyphs(), &[GlyphId::new(10)]);
    assert_eq!(positioning.mark2_coverage().glyphs(), &[GlyphId::new(20)]);

    let mark1 = positioning.mark1_record(GlyphId::new(10)).unwrap();
    assert_eq!(mark1.class(), 0);
    assert_eq!(mark1.anchor(), Anchor::coordinate(5, 15));

    let mark2 = positioning.mark2_record(GlyphId::new(20)).unwrap();
    assert_eq!(mark2.anchor(0), Some(Anchor::coordinate(30, 0)));
}

#[test]
fn decodes_null_mark2_anchor_as_absent() {
    let mut bytes = sample();
    bytes[39] = 0x00; // Mark2Array anchorOffset = null
    let positioning = decode(&bytes).unwrap();
    let mark2 = positioning.mark2_record(GlyphId::new(20)).unwrap();
    assert_eq!(mark2.len(), 1);
    assert_eq!(mark2.anchor(0), None);
    assert_eq!(mark2.anchors(), &[None]);
}

#[test]
fn rejects_unsupported_subtable_format() {
    let diagnostic = decode(&[0x00, 0x02, 0x00, 0x00, 0x00, 0x00]).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposMarkToMarkFormat { format: 2 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
}

#[test]
fn rejects_mark1_coverage_offset_inside_the_header() {
    let mut bytes = sample();
    bytes[3] = 0x02; // mark1CoverageOffset = 2, inside the 12-byte header
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::MarkToMarkMark1Coverage,
            offset: 2,
            minimum: 12,
            container_length: 46,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(2, 4));
}

#[test]
fn rejects_mark_class_outside_declared_range() {
    let mut bytes = sample();
    bytes[27] = 0x01; // MarkRecord markClass = 1, not below markClassCount = 1
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidGposMarkToMarkClass {
            class: 1,
            mark_class_count: 1,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(26, 28));
}

#[test]
fn rejects_mark1_count_disagreeing_with_coverage() {
    let mut bytes = sample();
    bytes[25] = 0x00; // MarkArray markCount = 0, but mark1Coverage lists one glyph
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGposMarkToMarkMark1Count {
            inputs: 1,
            records: 0,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(8, 10));
}

#[test]
fn rejects_mark2_count_disagreeing_with_coverage() {
    let mut bytes = sample();
    bytes[37] = 0x00; // Mark2Array mark2Count = 0, but mark2Coverage lists one glyph
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGposMarkToMarkMark2Count {
            inputs: 1,
            records: 0,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(10, 12));
}

#[test]
fn propagates_unsupported_anchor_format() {
    let mut bytes = sample();
    bytes[31] = 0x09; // Anchor A format = 9
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposAnchorFormat { format: 9 })
    );
}

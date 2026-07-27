use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::{BaseRecord, MarkBaseAttachment, MarkRecord, MarkToBasePositioning};
use crate::{Anchor, Coverage};

fn decode(bytes: &[u8]) -> glyphweave_sfnt::Decoded<MarkToBasePositioning> {
    MarkToBasePositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
}

/// A well-formed `MarkBasePos` format 1 subtable covering marks 10, 11 and base 20.
///
/// Mark 10 has class 0, mark 11 has class 1; the base supplies an anchor for class 0 and a null
/// anchor for class 1.
const SUBTABLE: [u8; 60] = [
    0x00, 0x01, // posFormat = 1
    0x00, 0x0c, // markCoverageOffset = 12
    0x00, 0x14, // baseCoverageOffset = 20
    0x00, 0x02, // markClassCount = 2
    0x00, 0x1a, // markArrayOffset = 26
    0x00, 0x30, // baseArrayOffset = 48
    0x00, 0x01, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x0b, // mark Coverage: glyphs 10, 11
    0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // base Coverage: glyph 20
    0x00, 0x02, // MarkArray markCount = 2
    0x00, 0x00, 0x00, 0x0a, // mark 0: class 0, anchorOffset 10
    0x00, 0x01, 0x00, 0x10, // mark 1: class 1, anchorOffset 16
    0x00, 0x01, 0x00, 0x64, 0x00, 0x00, // mark anchor 0: (100, 0)
    0x00, 0x01, 0xff, 0xce, 0x00, 0x0a, // mark anchor 1: (-50, 10)
    0x00, 0x01, // BaseArray baseCount = 1
    0x00, 0x06, 0x00, 0x00, // base 0: class 0 -> offset 6, class 1 -> null
    0x00, 0x01, 0x00, 0xc8, 0x01, 0x2c, // base anchor: (200, 300)
];

#[test]
fn records_expose_their_fields() {
    let mark = MarkRecord::new(1, Anchor::coordinate(3, 4));
    assert_eq!(mark.class(), 1);
    assert_eq!(mark.anchor(), Anchor::coordinate(3, 4));

    let base = BaseRecord::new([Some(Anchor::coordinate(5, 6)), None]);
    assert_eq!(base.len(), 2);
    assert!(!base.is_empty());
    assert_eq!(base.anchors().len(), 2);
    assert_eq!(base.anchor(0), Some(Anchor::coordinate(5, 6)));
    assert_eq!(base.anchor(1), None);
    assert_eq!(base.anchor(2), None);
    assert!(BaseRecord::default().is_empty());

    let attachment = MarkBaseAttachment::new(2, Anchor::coordinate(7, 8), None);
    assert_eq!(attachment.mark_class(), 2);
    assert_eq!(attachment.mark_anchor(), Anchor::coordinate(7, 8));
    assert_eq!(attachment.base_anchor(), None);
}

#[test]
fn model_resolves_attachments_through_both_coverages() {
    let marks = Coverage::from_glyphs([GlyphId::new(10), GlyphId::new(11)]);
    let bases = Coverage::from_glyphs([GlyphId::new(20)]);
    let positioning = MarkToBasePositioning::new(
        marks,
        bases,
        2,
        [
            MarkRecord::new(0, Anchor::coordinate(30, 0)),
            MarkRecord::new(1, Anchor::coordinate(-50, 10)),
        ],
        [BaseRecord::new([Some(Anchor::coordinate(200, 300)), None])],
    );
    assert_eq!(positioning.mark_class_count(), 2);
    assert_eq!(positioning.marks().len(), 2);
    assert_eq!(positioning.bases().len(), 1);
    assert_eq!(positioning.mark_coverage().len(), 2);
    assert_eq!(positioning.base_coverage().len(), 1);

    let first = positioning
        .attachment(GlyphId::new(10), GlyphId::new(20))
        .unwrap();
    assert_eq!(first.mark_class(), 0);
    assert_eq!(first.mark_anchor(), Anchor::coordinate(30, 0));
    assert_eq!(first.base_anchor(), Some(Anchor::coordinate(200, 300)));

    let second = positioning
        .attachment(GlyphId::new(11), GlyphId::new(20))
        .unwrap();
    assert_eq!(second.mark_class(), 1);
    assert_eq!(second.base_anchor(), None);

    assert!(
        positioning
            .attachment(GlyphId::new(12), GlyphId::new(20))
            .is_none()
    );
    assert!(
        positioning
            .attachment(GlyphId::new(10), GlyphId::new(21))
            .is_none()
    );
    assert!(
        MarkToBasePositioning::<GlyphId>::default()
            .attachment(GlyphId::new(0), GlyphId::new(0))
            .is_none()
    );
}

#[test]
fn decodes_format_1_mark_and_base_anchors() {
    let positioning = decode(&SUBTABLE).unwrap();
    assert_eq!(
        positioning.mark_coverage().glyphs(),
        &[GlyphId::new(10), GlyphId::new(11)]
    );
    assert_eq!(positioning.base_coverage().glyphs(), &[GlyphId::new(20)]);
    assert_eq!(positioning.mark_class_count(), 2);

    let first = positioning
        .attachment(GlyphId::new(10), GlyphId::new(20))
        .unwrap();
    assert_eq!(first.mark_class(), 0);
    assert_eq!(first.mark_anchor(), Anchor::coordinate(100, 0));
    assert_eq!(first.base_anchor(), Some(Anchor::coordinate(200, 300)));

    let second = positioning
        .attachment(GlyphId::new(11), GlyphId::new(20))
        .unwrap();
    assert_eq!(second.mark_class(), 1);
    assert_eq!(second.mark_anchor(), Anchor::coordinate(-50, 10));
    assert_eq!(second.base_anchor(), None);
}

#[test]
fn rejects_unsupported_subtable_format() {
    let diagnostic = decode(&[0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposMarkToBaseFormat { format: 2 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
}

#[test]
fn rejects_mark_class_at_or_above_mark_class_count() {
    let mut bytes = SUBTABLE;
    bytes[29] = 0x02; // mark 0 class 2 == markClassCount
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidGposMarkToBaseClass {
            class: 2,
            mark_class_count: 2,
        })
    );
}

#[test]
fn rejects_mark_count_disagreeing_with_coverage() {
    let mut bytes = SUBTABLE;
    bytes[3] = 0x14; // markCoverageOffset = base Coverage (single glyph) while MarkArray has two
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGposMarkToBaseMarkCount {
            coverage: 1,
            records: 2,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(8, 10));
}

#[test]
fn rejects_base_count_disagreeing_with_coverage() {
    let mut bytes = SUBTABLE;
    bytes[5] = 0x0c; // baseCoverageOffset = mark Coverage (two glyphs) while BaseArray has one record
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGposMarkToBaseBaseCount {
            coverage: 2,
            records: 1,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(10, 12));
}

#[test]
fn rejects_mark_array_offset_inside_the_header() {
    let mut bytes = SUBTABLE;
    bytes[9] = 0x02; // markArrayOffset = 2, inside the 12-byte header
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::GposMarkToBaseMarkArray,
            offset: 2,
            minimum: 12,
            container_length: 60,
        })
    );
}

#[test]
fn propagates_unsupported_anchor_format() {
    let mut bytes = SUBTABLE;
    bytes[36] = 0x00;
    bytes[37] = 0x09; // mark anchor 0 declares unsupported format 9
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposAnchorFormat { format: 9 })
    );
}

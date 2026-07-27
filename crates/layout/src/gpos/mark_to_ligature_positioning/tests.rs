use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::{LigatureAttachment, MarkLigaturePositioning, MarkLigatureRecord};
use crate::{Anchor, Coverage};

fn decode(bytes: &[u8]) -> glyphweave_sfnt::Decoded<MarkLigaturePositioning> {
    MarkLigaturePositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
}

#[test]
fn records_expose_class_anchor_and_grid() {
    let record = MarkLigatureRecord::new(1, Anchor::coordinate(5, 6));
    assert_eq!(record.class(), 1);
    assert_eq!(record.anchor(), Anchor::coordinate(5, 6));

    let attach = LigatureAttachment::new([
        vec![Some(Anchor::coordinate(100, 0)), None],
        vec![None, Some(Anchor::coordinate(300, 0))],
    ]);
    assert_eq!(attach.component_count(), 2);
    assert_eq!(attach.components().len(), 2);
    assert!(!attach.is_empty());
    assert!(LigatureAttachment::default().is_empty());
    assert_eq!(attach.anchor(0, 0), Some(Anchor::coordinate(100, 0)));
    assert_eq!(attach.anchor(0, 1), None);
    assert_eq!(attach.anchor(1, 1), Some(Anchor::coordinate(300, 0)));
    assert_eq!(attach.anchor(2, 0), None);
    assert_eq!(attach.anchor(0, 9), None);
}

#[test]
fn model_resolves_records_through_coverage() {
    let positioning = MarkLigaturePositioning::new(
        Coverage::from_glyphs([GlyphId::new(20)]),
        Coverage::from_glyphs([GlyphId::new(7)]),
        2,
        [MarkLigatureRecord::new(1, Anchor::coordinate(10, 40))],
        [LigatureAttachment::new([vec![
            None,
            Some(Anchor::coordinate(9, 8)),
        ]])],
    );

    assert_eq!(positioning.mark_class_count(), 2);
    assert_eq!(positioning.marks().len(), 1);
    assert_eq!(positioning.ligatures().len(), 1);
    assert_eq!(positioning.mark_coverage().len(), 1);
    assert_eq!(positioning.ligature_coverage().len(), 1);
    assert!(!positioning.is_empty());
    assert!(MarkLigaturePositioning::<GlyphId>::default().is_empty());

    let mark = positioning.mark(GlyphId::new(20)).unwrap();
    assert_eq!(mark.class(), 1);
    assert_eq!(mark.anchor(), Anchor::coordinate(10, 40));
    assert!(positioning.mark(GlyphId::new(21)).is_none());

    let ligature = positioning.ligature(GlyphId::new(7)).unwrap();
    assert_eq!(
        ligature.anchor(0, mark.class()),
        Some(Anchor::coordinate(9, 8))
    );
    assert_eq!(ligature.anchor(0, 0), None);
    assert!(positioning.ligature(GlyphId::new(8)).is_none());
}

#[test]
fn decodes_format_1_mark_and_ligature_grid() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0c, // markCoverageOffset = 12
        0x00, 0x12, // ligatureCoverageOffset = 18
        0x00, 0x02, // markClassCount = 2
        0x00, 0x18, // markArrayOffset = 24
        0x00, 0x24, // ligatureArrayOffset = 36
        0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // mark Coverage: glyph 20
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // ligature Coverage: glyph 7
        0x00, 0x01, // MarkArray markCount = 1
        0x00, 0x00, // record 0 markClass = 0
        0x00, 0x06, // record 0 markAnchorOffset = 6
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x28, // mark anchor (10, 40)
        0x00, 0x01, // LigatureArray ligatureCount = 1
        0x00, 0x04, // ligatureAttachOffset = 4
        0x00, 0x02, // LigatureAttach componentCount = 2
        0x00, 0x0a, // component 0 class 0 anchorOffset = 10
        0x00, 0x00, // component 0 class 1 anchorOffset = null
        0x00, 0x00, // component 1 class 0 anchorOffset = null
        0x00, 0x10, // component 1 class 1 anchorOffset = 16
        0x00, 0x01, 0x00, 0x64, 0x00, 0x00, // anchor A (100, 0)
        0x00, 0x01, 0x01, 0x2c, 0x00, 0x00, // anchor B (300, 0)
    ];
    let positioning = decode(&bytes).unwrap();
    assert_eq!(positioning.mark_coverage().glyphs(), &[GlyphId::new(20)]);
    assert_eq!(positioning.ligature_coverage().glyphs(), &[GlyphId::new(7)]);
    assert_eq!(positioning.mark_class_count(), 2);

    let mark = positioning.mark(GlyphId::new(20)).unwrap();
    assert_eq!(mark.class(), 0);
    assert_eq!(mark.anchor(), Anchor::coordinate(10, 40));

    let ligature = positioning.ligature(GlyphId::new(7)).unwrap();
    assert_eq!(ligature.component_count(), 2);
    assert_eq!(ligature.anchor(0, 0), Some(Anchor::coordinate(100, 0)));
    assert_eq!(ligature.anchor(0, 1), None);
    assert_eq!(ligature.anchor(1, 0), None);
    assert_eq!(ligature.anchor(1, 1), Some(Anchor::coordinate(300, 0)));
}

#[test]
fn rejects_unsupported_subtable_format() {
    let bytes = [
        0x00, 0x02, // posFormat = 2
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposMarkLigatureFormat { format: 2 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
}

#[test]
fn rejects_mark_coverage_offset_inside_the_header() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x02, // markCoverageOffset = 2 (inside header)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::MarkLigatureMarkCoverage,
            offset: 2,
            minimum: 12,
            container_length: 12,
        })
    );
}

#[test]
fn rejects_out_of_range_mark_class() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0c, // markCoverageOffset = 12
        0x00, 0x12, // ligatureCoverageOffset = 18
        0x00, 0x01, // markClassCount = 1
        0x00, 0x18, // markArrayOffset = 24
        0x00, 0x00, // ligatureArrayOffset (unreached)
        0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // mark Coverage: glyph 20
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // ligature Coverage: glyph 7
        0x00, 0x01, // markCount = 1
        0x00, 0x05, // record 0 markClass = 5 (>= markClassCount 1)
        0x00, 0x06, // record 0 markAnchorOffset = 6
        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // anchor (0, 0)
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MarkLigatureMarkClassRange {
            class: 5,
            class_count: 1,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(26, 28));
}

#[test]
fn rejects_mark_count_disagreeing_with_coverage() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0c, // markCoverageOffset = 12
        0x00, 0x12, // ligatureCoverageOffset = 18
        0x00, 0x01, // markClassCount = 1
        0x00, 0x18, // markArrayOffset = 24
        0x00, 0x00, // ligatureArrayOffset (unreached)
        0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // mark Coverage: glyph 20 (1 glyph)
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // ligature Coverage: glyph 7
        0x00, 0x02, // markCount = 2 (Coverage has 1)
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedMarkLigatureMarkCount {
            coverage: 1,
            marks: 2,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(24, 26));
}

#[test]
fn rejects_ligature_count_disagreeing_with_coverage() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0c, // markCoverageOffset = 12
        0x00, 0x12, // ligatureCoverageOffset = 18
        0x00, 0x01, // markClassCount = 1
        0x00, 0x18, // markArrayOffset = 24
        0x00, 0x24, // ligatureArrayOffset = 36
        0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // mark Coverage: glyph 20
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // ligature Coverage: glyph 7 (1 glyph)
        0x00, 0x01, // markCount = 1
        0x00, 0x00, // record 0 markClass = 0
        0x00, 0x06, // record 0 markAnchorOffset = 6
        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // mark anchor (0, 0)
        0x00, 0x02, // ligatureCount = 2 (Coverage has 1)
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedMarkLigatureLigatureCount {
            coverage: 1,
            ligatures: 2,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(36, 38));
}

#[test]
fn propagates_unsupported_ligature_anchor_format() {
    let bytes = [
        0x00, 0x01, // posFormat = 1
        0x00, 0x0c, // markCoverageOffset = 12
        0x00, 0x12, // ligatureCoverageOffset = 18
        0x00, 0x01, // markClassCount = 1
        0x00, 0x18, // markArrayOffset = 24
        0x00, 0x24, // ligatureArrayOffset = 36
        0x00, 0x01, 0x00, 0x01, 0x00, 0x14, // mark Coverage: glyph 20
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // ligature Coverage: glyph 7
        0x00, 0x01, // markCount = 1
        0x00, 0x00, // record 0 markClass = 0
        0x00, 0x06, // record 0 markAnchorOffset = 6
        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, // mark anchor (0, 0)
        0x00, 0x01, // ligatureCount = 1
        0x00, 0x04, // ligatureAttachOffset = 4
        0x00, 0x01, // componentCount = 1
        0x00, 0x04, // component 0 class 0 anchorOffset = 4
        0x00, 0x09, 0x00, 0x00, // anchor with unsupported format 9
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposAnchorFormat { format: 9 })
    );
}

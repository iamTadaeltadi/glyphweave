use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::{CursivePositioning, EntryExit};
use crate::{Anchor, Coverage};

fn decode(bytes: &[u8]) -> glyphweave_sfnt::Decoded<CursivePositioning> {
    CursivePositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
}

#[test]
fn entry_exit_records_expose_optional_anchors() {
    let full = EntryExit::new(
        Some(Anchor::coordinate(1, 2)),
        Some(Anchor::coordinate(3, 4)),
    );
    assert_eq!(full.entry(), Some(Anchor::coordinate(1, 2)));
    assert_eq!(full.exit(), Some(Anchor::coordinate(3, 4)));
    assert!(!full.is_empty());

    assert!(EntryExit::default().is_empty());
    assert!(EntryExit::new(None, None).is_empty());
}

#[test]
fn model_resolves_records_through_coverage() {
    let coverage = Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(9)]);
    let cursive = CursivePositioning::new(
        coverage,
        [
            EntryExit::new(Some(Anchor::coordinate(10, 0)), None),
            EntryExit::new(None, Some(Anchor::coordinate(0, 20))),
        ],
    );
    assert_eq!(cursive.len(), 2);
    assert!(!cursive.is_empty());
    assert_eq!(cursive.records().len(), 2);
    assert_eq!(cursive.coverage().len(), 2);
    assert_eq!(
        cursive.entry_exit(GlyphId::new(4)).unwrap().entry(),
        Some(Anchor::coordinate(10, 0))
    );
    assert!(
        cursive
            .entry_exit(GlyphId::new(9))
            .unwrap()
            .entry()
            .is_none()
    );
    assert!(cursive.entry_exit(GlyphId::new(5)).is_none());
    assert!(CursivePositioning::<GlyphId>::default().is_empty());
}

#[test]
fn decodes_format_1_entry_and_exit_anchors() {
    let bytes = [
        0x00, 0x01, // posFormat
        0x00, 0x0e, // coverageOffset = 14
        0x00, 0x02, // entryExitCount = 2
        0x00, 0x16, // record 0 entryAnchorOffset = 22
        0x00, 0x00, // record 0 exitAnchorOffset = null
        0x00, 0x00, // record 1 entryAnchorOffset = null
        0x00, 0x1c, // record 1 exitAnchorOffset = 28
        0x00, 0x01, 0x00, 0x02, 0x00, 0x07, 0x00, 0x09, // Coverage: glyphs 7, 9
        0x00, 0x01, 0x00, 0x32, 0x00, 0x00, // anchor at 22: (50, 0)
        0x00, 0x01, 0xff, 0xf6, 0x00, 0x64, // anchor at 28: (-10, 100)
    ];
    let cursive = decode(&bytes).unwrap();
    assert_eq!(
        cursive.coverage().glyphs(),
        &[GlyphId::new(7), GlyphId::new(9)]
    );

    let first = cursive.entry_exit(GlyphId::new(7)).unwrap();
    assert_eq!(first.entry(), Some(Anchor::coordinate(50, 0)));
    assert!(first.exit().is_none());

    let second = cursive.entry_exit(GlyphId::new(9)).unwrap();
    assert!(second.entry().is_none());
    assert_eq!(second.exit(), Some(Anchor::coordinate(-10, 100)));
}

#[test]
fn rejects_unsupported_subtable_format() {
    let diagnostic = decode(&[0x00, 0x02, 0x00, 0x00, 0x00, 0x00]).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposCursiveFormat { format: 2 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
}

#[test]
fn rejects_coverage_offset_inside_the_header() {
    let diagnostic = decode(&[0x00, 0x01, 0x00, 0x02, 0x00, 0x00]).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::GposCursiveCoverage,
            offset: 2,
            minimum: 6,
            container_length: 6,
        })
    );
}

#[test]
fn rejects_record_count_disagreeing_with_coverage() {
    let bytes = [
        0x00, 0x01, // posFormat
        0x00, 0x0e, // coverageOffset = 14
        0x00, 0x02, // entryExitCount = 2
        0x00, 0x00, 0x00, 0x00, // record 0 (null, null)
        0x00, 0x00, 0x00, 0x00, // record 1 (null, null)
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // Coverage: single glyph 7
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGposEntryExitCount {
            inputs: 1,
            records: 2,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(4, 6));
}

#[test]
fn propagates_unsupported_anchor_format() {
    let bytes = [
        0x00, 0x01, // posFormat
        0x00, 0x0a, // coverageOffset = 10
        0x00, 0x01, // entryExitCount = 1
        0x00, 0x10, // record 0 entryAnchorOffset = 16
        0x00, 0x00, // record 0 exitAnchorOffset = null
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // Coverage: single glyph 7
        0x00, 0x09, 0x00, 0x00, // anchor at 16 with unsupported format 9
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposAnchorFormat { format: 9 })
    );
}

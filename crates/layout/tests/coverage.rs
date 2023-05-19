//! Semantic decoding tests for OpenType Coverage tables.

use glyphweave_layout::Coverage;
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause};

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

#[test]
fn rejects_unknown_format_at_the_selector() {
    let diagnostic = Coverage::decode(&[0xaa, 0x00, 0x09], 1).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(1, 3));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedCoverageFormat { format: 9 })
    );
}

#[test]
fn rejects_decode_offsets_beyond_the_input() {
    let diagnostic = Coverage::decode(&[0x00], 2).unwrap_err();
    assert_eq!(diagnostic.span(), ByteSpan::between(1, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::SpanOutOfBounds { end: 2, length: 1 }
    );
}

#[test]
fn rejects_duplicate_format_1_glyphs() {
    let bytes = [0x00, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x05];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(6, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::NonIncreasingCoverageGlyph {
            previous: GlyphId::new(5),
            current: GlyphId::new(5),
        })
    );
}

#[test]
fn rejects_decreasing_format_1_glyphs() {
    let bytes = [0x00, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x04];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::NonIncreasingCoverageGlyph {
            previous: GlyphId::new(5),
            current: GlyphId::new(4),
        })
    );
}

#[test]
fn decodes_format_2_ranges_into_sequential_indices() {
    let bytes = [
        0x00, 0x02, 0x00, 0x02, // format, rangeCount
        0x00, 0x03, 0x00, 0x05, 0x00, 0x00, // 3..=5 at index 0
        0x00, 0x09, 0x00, 0x0a, 0x00, 0x03, // 9..=10 at index 3
    ];
    let coverage = Coverage::decode(&bytes, 0).unwrap();

    assert_eq!(coverage.glyphs(), &[3, 4, 5, 9, 10].map(GlyphId::new));
    assert_eq!(coverage.coverage_index(GlyphId::new(10)), Some(4));
}

#[test]
fn accepts_distinct_adjacent_format_2_ranges() {
    let bytes = [
        0x00, 0x02, 0x00, 0x02, // format, rangeCount
        0x00, 0x03, 0x00, 0x04, 0x00, 0x00, // 3..=4
        0x00, 0x05, 0x00, 0x06, 0x00, 0x02, // 5..=6
    ];
    let coverage = Coverage::decode(&bytes, 0).unwrap();

    assert_eq!(coverage.glyphs(), &[3, 4, 5, 6].map(GlyphId::new));
}

#[test]
fn decodes_empty_format_2() {
    let coverage = Coverage::decode(&[0x00, 0x02, 0x00, 0x00], 0).unwrap();
    assert!(coverage.is_empty());
}

#[test]
fn rejects_reversed_coverage_range() {
    let bytes = [
        0x00, 0x02, 0x00, 0x01, // format, rangeCount
        0x00, 0x06, 0x00, 0x05, 0x00, 0x00,
    ];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(4, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidCoverageRange {
            start: GlyphId::new(6),
            end: GlyphId::new(5),
        })
    );
}

#[test]
fn rejects_overlapping_coverage_ranges() {
    let bytes = [
        0x00, 0x02, 0x00, 0x02, // format, rangeCount
        0x00, 0x03, 0x00, 0x05, 0x00, 0x00, // 3..=5
        0x00, 0x05, 0x00, 0x07, 0x00, 0x03, // overlaps at 5
    ];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(10, 12));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::OverlappingCoverageRange {
            previous_end: GlyphId::new(5),
            start: GlyphId::new(5),
        })
    );
}

#[test]
fn rejects_nonzero_first_coverage_index() {
    let bytes = [
        0x00, 0x02, 0x00, 0x01, // format, rangeCount
        0x00, 0x03, 0x00, 0x05, 0x00, 0x01,
    ];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(8, 10));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::DiscontinuousCoverageIndex {
            expected: 0,
            actual: 1,
        })
    );
}

#[test]
fn rejects_discontinuous_later_coverage_index() {
    let bytes = [
        0x00, 0x02, 0x00, 0x02, // format, rangeCount
        0x00, 0x03, 0x00, 0x05, 0x00, 0x00, // three indices
        0x00, 0x09, 0x00, 0x0a, 0x00, 0x04, // should begin at three
    ];
    let diagnostic = Coverage::decode(&bytes, 0).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(14, 16));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::DiscontinuousCoverageIndex {
            expected: 3,
            actual: 4,
        })
    );
}

#[test]
fn decodes_the_complete_glyph_id_domain() {
    let bytes = [
        0x00, 0x02, 0x00, 0x01, // format, rangeCount
        0x00, 0x00, 0xff, 0xff, 0x00, 0x00,
    ];
    let coverage = Coverage::decode(&bytes, 0).unwrap();

    assert_eq!(coverage.len(), 65_536);
    assert_eq!(coverage.glyph_at(u16::MAX), Some(GlyphId::new(u16::MAX)));
    assert_eq!(
        coverage.coverage_index(GlyphId::new(u16::MAX)),
        Some(u16::MAX)
    );
}

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

//! Semantic coverage for common TrueType Glyph Headers.

use glyphweave_outline::{GlyphBounds, GlyphHeader};
use glyphweave_sfnt::ByteSpan;

#[test]
fn decodes_a_simple_header_at_an_absolute_glyf_position() {
    let bytes = [
        0xaa, 0xbb, 0x00, 0x02, 0xff, 0xfd, 0xff, 0xfc, 0x00, 0x07, 0x00, 0x09, 0xcc,
    ];
    let span = ByteSpan::sized(2, 11).unwrap();

    let header = GlyphHeader::decode(&bytes, span).unwrap();

    assert_eq!(header.number_of_contours(), 2);
    assert_eq!(header.simple_contour_count(), Some(2));
    assert_eq!(header.bounds(), GlyphBounds::new(-3, -4, 7, 9).unwrap());
}

#[test]
fn classifies_negative_contour_markers_as_composite() {
    let bytes = [0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0];
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let header = GlyphHeader::decode(&bytes, span).unwrap();

    assert!(header.is_composite());
    assert_eq!(header.number_of_contours(), -1);
    assert_eq!(header.simple_contour_count(), None);
}

#[test]
fn preserves_degenerate_zero_bounds_for_an_empty_outline() {
    let bytes = [0; 10];
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let header = GlyphHeader::decode(&bytes, span).unwrap();

    assert_eq!(header.bounds(), GlyphBounds::new(0, 0, 0, 0).unwrap());
    assert_eq!(header.simple_contour_count(), Some(0));
}

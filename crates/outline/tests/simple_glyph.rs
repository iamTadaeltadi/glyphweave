//! Semantic coverage for canonical simple-glyph decoding.

use glyphweave_outline::{GlyphBounds, Point, SimpleGlyph};
use glyphweave_sfnt::ByteSpan;

fn packed_two_contour_glyph() -> Vec<u8> {
    let mut bytes = Vec::new();
    for value in [2, -5, 10, 20, 30] {
        bytes.extend_from_slice(&i16::to_be_bytes(value));
    }
    bytes.extend_from_slice(&u16::to_be_bytes(1));
    bytes.extend_from_slice(&u16::to_be_bytes(3));
    bytes.extend_from_slice(&u16::to_be_bytes(2));
    bytes.extend_from_slice(&[0xb0, 0x00]);
    bytes.extend_from_slice(&[0x37, 0x32, 0x35, 0x23]);
    bytes.extend_from_slice(&[10, 10, 25]);
    bytes.extend_from_slice(&[10, 20]);
    bytes.push(0);
    bytes
}

#[test]
fn expands_contours_flags_deltas_and_instructions_at_an_absolute_span() {
    let mut bytes = vec![0xaa, 0xbb];
    bytes.extend_from_slice(&packed_two_contour_glyph());
    bytes.push(0xcc);
    let span = ByteSpan::sized(2, 28).unwrap();

    let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

    let expected = [
        Point::new(10, 10, true),
        Point::new(20, 10, false),
        Point::new(20, 30, true),
        Point::new(-5, 30, true),
    ];
    assert_eq!(glyph.span(), span);
    assert_eq!(glyph.bounds(), GlyphBounds::new(-5, 10, 20, 30).unwrap());
    assert_eq!(glyph.contour_ends(), [2, 4]);
    assert_eq!(glyph.points(), expected);
    assert_eq!(glyph.contour(0), Some(&expected[0..2]));
    assert_eq!(glyph.contour(1), Some(&expected[2..4]));
    assert_eq!(glyph.instructions(), [0xb0, 0x00]);
    assert!(!glyph.has_overlapping_contours());
}

#[test]
fn expands_a_repeated_first_flag_and_preserves_overlap_identity() {
    let mut bytes = Vec::new();
    for value in [1i16, 0, 0, 0, 0] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes.extend_from_slice(&2u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x79, 2]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.contour_ends(), [3]);
    assert_eq!(glyph.points(), [Point::new(0, 0, true); 3]);
    assert!(glyph.instructions().is_empty());
    assert!(glyph.has_overlapping_contours());
}

#[test]
fn decodes_zero_contours_with_optional_phantom_point_instructions() {
    let header_only = [0; 10];
    let bare_span = ByteSpan::sized(0, header_only.len()).unwrap();
    let bare = SimpleGlyph::decode(&header_only, bare_span).unwrap();

    assert_eq!(bare.contour_count(), 0);
    assert!(bare.points().is_empty());
    assert!(bare.instructions().is_empty());

    let mut instructed = header_only.to_vec();
    instructed.extend_from_slice(&2u16.to_be_bytes());
    instructed.extend_from_slice(&[0xb0, 0x01]);
    let instructed_span = ByteSpan::sized(0, instructed.len()).unwrap();
    let decoded = SimpleGlyph::decode(&instructed, instructed_span).unwrap();

    assert_eq!(decoded.contour_count(), 0);
    assert!(decoded.points().is_empty());
    assert_eq!(decoded.instructions(), [0xb0, 0x01]);
}

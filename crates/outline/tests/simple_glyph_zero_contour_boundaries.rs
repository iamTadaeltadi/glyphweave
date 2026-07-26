//! Boundary coverage for zero-contour simple glyphs.

use glyphweave_outline::{GlyphBounds, SimpleGlyph};
use glyphweave_sfnt::{ByteSpan, Cause};

#[test]
fn a_byte_after_zero_contour_header_starts_an_instruction_length() {
    let mut bytes = vec![0; 10];
    bytes.push(0);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("bytes after the header begin a two-byte instruction length");

    assert_eq!(diagnostic.span(), ByteSpan::sized(10, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1
        }
    );
}

#[test]
fn explicit_zero_instruction_length_is_a_complete_zero_contour_glyph() {
    let mut bytes = vec![0; 10];
    bytes.extend_from_slice(&0u16.to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.span(), span);
    assert_eq!(glyph.contour_count(), 0);
    assert!(glyph.instructions().is_empty());
}

#[test]
fn ordered_zero_contour_bounds_are_preserved_without_point_extrema() {
    let mut bytes = Vec::new();
    for value in [0i16, -5, -6, 7, 8] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.bounds(), GlyphBounds::new(-5, -6, 7, 8).unwrap());
    assert!(glyph.points().is_empty());
}

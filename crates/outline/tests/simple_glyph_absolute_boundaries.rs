//! Absolute-span diagnostics for malformed simple glyphs.

use glyphweave_outline::SimpleGlyph;
use glyphweave_sfnt::{ByteSpan, Cause, CoordinateAxis, OutlineCause};

#[test]
fn flag_repeat_failure_is_located_inside_an_absolute_glyph_span() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    for value in [1i16, 0, 0, 0, 0] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x39, 2]);
    let span = ByteSpan::sized(3, 16).unwrap();
    let diagnostic =
        SimpleGlyph::decode(&bytes, span).expect_err("the run expands past two points");

    assert_eq!(diagnostic.span(), ByteSpan::sized(18, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::SimpleFlagRepeatOutOfBounds {
            point_index: 0,
            additional_repetitions: 2,
            point_count: 2,
        })
    );
}

#[test]
fn coordinate_overflow_is_located_inside_an_absolute_glyph_span() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    for value in [1i16, 0, 0, 0, 0] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x21, 0x21]);
    bytes.extend_from_slice(&i16::MAX.to_be_bytes());
    bytes.extend_from_slice(&1i16.to_be_bytes());
    let span = ByteSpan::sized(3, 20).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span).expect_err("the second x sum overflows");

    assert_eq!(diagnostic.span(), ByteSpan::sized(21, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::SimpleCoordinateOverflow {
            axis: CoordinateAxis::Horizontal,
            point_index: 1,
            previous: i16::MAX,
            delta: 1,
        })
    );
}

#[test]
fn bounds_mismatch_is_located_inside_an_absolute_glyph_span() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    for value in [1i16, 0, 0, 0, 0] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x33, 1]);
    let span = ByteSpan::sized(3, 16).unwrap();
    let diagnostic =
        SimpleGlyph::decode(&bytes, span).expect_err("point one is outside zero bounds");

    assert_eq!(diagnostic.span(), ByteSpan::sized(5, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::SimpleGlyphBoundsMismatch {
            recorded: [0, 0, 0, 0],
            calculated: [1, 0, 1, 0],
        })
    );
}

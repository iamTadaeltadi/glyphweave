//! Malformed-stream coverage for simple TrueType glyphs.

use glyphweave_outline::SimpleGlyph;
use glyphweave_sfnt::{ByteSpan, Cause, CoordinateAxis, OutlineCause};

fn header(number_of_contours: i16, bounds: [i16; 4]) -> Vec<u8> {
    let mut bytes = number_of_contours.to_be_bytes().to_vec();
    for value in bounds {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes
}

#[test]
fn composite_headers_are_rejected_by_the_simple_decoder() {
    let bytes = [0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0];
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic =
        SimpleGlyph::decode(&bytes, span).expect_err("negative contour counts are composite");

    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::CompositeGlyphForSimpleDecoder {
            number_of_contours: -1
        })
    );
}

#[test]
fn contour_end_points_must_increase_strictly() {
    let mut bytes = header(2, [0; 4]);
    bytes.extend_from_slice(&3u16.to_be_bytes());
    bytes.extend_from_slice(&3u16.to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("two contours cannot share their inclusive endpoint");

    assert_eq!(diagnostic.span(), ByteSpan::sized(12, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::NonIncreasingContourEnd {
            contour_index: 1,
            end_point: 3,
            previous_end_point: 3,
        })
    );
}

#[test]
fn instruction_program_cannot_cross_the_glyph_span() {
    let mut bytes = header(1, [0; 4]);
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&2u16.to_be_bytes());
    bytes.push(0xb0);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("the declared two-byte instruction program is truncated");

    assert_eq!(diagnostic.span(), ByteSpan::sized(14, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1
        }
    );
}

#[test]
fn flag_repeat_cannot_expand_past_the_point_count() {
    let mut bytes = header(1, [0; 4]);
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x39, 2]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("three logical flags cannot describe two points");

    assert_eq!(diagnostic.span(), ByteSpan::sized(15, 1));
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
fn reserved_simple_flag_bit_is_rejected_before_coordinates() {
    let mut bytes = header(1, [0; 4]);
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.push(0xb1);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic =
        SimpleGlyph::decode(&bytes, span).expect_err("bit seven is reserved by OpenType");

    assert_eq!(diagnostic.span(), ByteSpan::sized(14, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::ReservedSimpleGlyphFlag {
            point_index: 0,
            flag: 0xb1,
        })
    );
}

#[test]
fn overlap_marker_is_valid_only_on_the_first_physical_flag() {
    let mut bytes = header(1, [0; 4]);
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x31, 0x71]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("the second physical flag cannot introduce overlap identity");

    assert_eq!(diagnostic.span(), ByteSpan::sized(15, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::MisplacedSimpleOverlapFlag { point_index: 1 })
    );
}

#[test]
fn accumulated_coordinates_must_remain_signed_design_units() {
    let mut bytes = header(1, [0; 4]);
    bytes.extend_from_slice(&1u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x21, 0x21]);
    bytes.extend_from_slice(&i16::MAX.to_be_bytes());
    bytes.extend_from_slice(&1i16.to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("the second x delta exceeds the absolute coordinate domain");

    assert_eq!(diagnostic.span(), ByteSpan::sized(18, 2));
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
fn recorded_bounds_must_equal_decoded_point_extrema() {
    let mut bytes = header(1, [0; 4]);
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&0u16.to_be_bytes());
    bytes.extend_from_slice(&[0x33, 1]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = SimpleGlyph::decode(&bytes, span)
        .expect_err("the only point lies outside the recorded zero bounds");

    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::SimpleGlyphBoundsMismatch {
            recorded: [0, 0, 0, 0],
            calculated: [1, 0, 1, 0],
        })
    );
}

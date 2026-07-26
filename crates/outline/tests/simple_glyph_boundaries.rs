//! Malformed-stream coverage for simple TrueType glyphs.

use glyphweave_outline::SimpleGlyph;
use glyphweave_sfnt::{ByteSpan, Cause, OutlineCause};

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

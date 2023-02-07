//! Malformed-stream coverage for composite TrueType glyphs.

use glyphweave_outline::CompositeGlyph;
use glyphweave_sfnt::{ByteSpan, Cause, OutlineCause};

fn header(number_of_contours: i16) -> Vec<u8> {
    let mut bytes = number_of_contours.to_be_bytes().to_vec();
    bytes.extend([0; 8]);
    bytes
}

#[test]
fn simple_headers_are_rejected_by_the_composite_decoder() {
    let bytes = [0; 10];
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic =
        CompositeGlyph::decode(&bytes, span).expect_err("zero contours identify a simple glyph");

    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::SimpleGlyphForCompositeDecoder {
            number_of_contours: 0,
        })
    );
}

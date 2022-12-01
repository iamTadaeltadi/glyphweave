//! Malformed-stream coverage for simple TrueType glyphs.

use glyphweave_outline::SimpleGlyph;
use glyphweave_sfnt::{ByteSpan, Cause, OutlineCause};

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

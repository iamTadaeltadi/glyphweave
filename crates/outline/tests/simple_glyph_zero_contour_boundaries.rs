//! Boundary coverage for zero-contour simple glyphs.

use glyphweave_outline::SimpleGlyph;
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

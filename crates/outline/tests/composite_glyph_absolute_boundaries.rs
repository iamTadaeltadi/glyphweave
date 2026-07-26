//! Absolute-span diagnostics for composite-glyph failures.

use glyphweave_outline::CompositeGlyph;
use glyphweave_sfnt::{ByteSpan, Cause, OutlineCause};

#[test]
fn reserved_flags_are_located_inside_an_absolute_glyph_span() {
    let mut bytes = vec![0xaa; 5];
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x20, 0x00]);
    bytes.extend([0xbb; 4]);
    let glyph_span = ByteSpan::between(5, 17).unwrap();

    let diagnostic = CompositeGlyph::decode(&bytes, glyph_span)
        .expect_err("the component flag word sets reserved bit thirteen");

    assert_eq!(diagnostic.span(), ByteSpan::between(15, 17));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::ReservedCompositeGlyphFlags {
            component_index: 0,
            flags: 0x2000,
        })
    );
}

#[test]
fn instruction_failures_are_located_inside_an_absolute_glyph_span() {
    let mut bytes = vec![0xaa; 5];
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x01, 0x02, 0x00, 0x01, 0x00, 0x00]);
    bytes.extend([0x00, 0x02, 0xb0]);
    bytes.extend([0xbb; 4]);
    let glyph_span = ByteSpan::between(5, 24).unwrap();

    let diagnostic = CompositeGlyph::decode(&bytes, glyph_span)
        .expect_err("neighboring glyph bytes cannot complete parent instructions");

    assert_eq!(diagnostic.span(), ByteSpan::between(23, 24));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

#[test]
fn padding_failures_are_located_inside_an_absolute_glyph_span() {
    let mut bytes = vec![0xaa; 5];
    bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
    bytes.extend([0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x7f]);
    bytes.extend([0xbb; 4]);
    let glyph_span = ByteSpan::between(5, 22).unwrap();

    let diagnostic = CompositeGlyph::decode(&bytes, glyph_span)
        .expect_err("a neighboring glyph does not change a nonzero pad");

    assert_eq!(diagnostic.span(), ByteSpan::between(21, 22));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::InvalidCompositeGlyphPadding { value: 0x7f })
    );
}

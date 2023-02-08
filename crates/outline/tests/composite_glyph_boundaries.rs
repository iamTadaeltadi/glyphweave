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

#[test]
fn every_reserved_component_flag_is_rejected() {
    for flags in [0x0010_u16, 0x2000, 0x4000, 0x8000] {
        let mut bytes = header(-1);
        bytes.extend(flags.to_be_bytes());
        let span = ByteSpan::sized(0, bytes.len()).unwrap();
        let diagnostic = CompositeGlyph::decode(&bytes, span)
            .expect_err("reserved component flags must remain zero");

        assert_eq!(diagnostic.span(), ByteSpan::sized(10, 2));
        assert_eq!(
            diagnostic.cause(),
            Cause::Outline(OutlineCause::ReservedCompositeGlyphFlags {
                component_index: 0,
                flags,
            })
        );
    }
}

#[test]
fn component_transform_encodings_are_mutually_exclusive() {
    for flags in [0x004a_u16, 0x008a, 0x00c2] {
        let mut bytes = header(-1);
        bytes.extend(flags.to_be_bytes());
        let span = ByteSpan::sized(0, bytes.len()).unwrap();
        let diagnostic = CompositeGlyph::decode(&bytes, span)
            .expect_err("one component cannot select two transform layouts");

        assert_eq!(diagnostic.span(), ByteSpan::sized(10, 2));
        assert_eq!(
            diagnostic.cause(),
            Cause::Outline(OutlineCause::ConflictingCompositeTransforms {
                component_index: 0,
                flags,
            })
        );
    }
}

#[test]
fn scaled_and_unscaled_offset_modes_conflict() {
    let flags = 0x1802_u16;
    let mut bytes = header(-1);
    bytes.extend(flags.to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("a component offset cannot be both scaled and unscaled");

    assert_eq!(diagnostic.span(), ByteSpan::sized(10, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::ConflictingComponentOffsetModes {
            component_index: 0,
            flags,
        })
    );
}

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

#[test]
fn first_component_must_use_an_offset() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x00]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("point alignment requires points from an earlier child");

    assert_eq!(diagnostic.span(), ByteSpan::sized(10, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::FirstCompositeComponentUsesPoints)
    );
}

#[test]
fn overlap_marker_is_valid_only_on_the_first_component() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x22, 0x00, 0x01, 0x00, 0x00]);
    bytes.extend([0x04, 0x02]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("a late overlap marker has ambiguous glyph-wide meaning");

    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::MisplacedCompositeOverlapFlag { component_index: 1 })
    );
}

#[test]
fn every_strict_prefix_of_a_complete_composite_is_rejected() {
    let mut complete = header(-1);
    complete.extend([0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0xff, 0xff]);

    for length in 1..complete.len() {
        let span = ByteSpan::sized(0, length).unwrap();
        let diagnostic = CompositeGlyph::decode(&complete[..length], span)
            .expect_err("every encoded composite field must fit inside the glyph span");

        assert!(matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }));
        assert_eq!(diagnostic.span().unwrap().end(), length);
    }
}

#[test]
fn uniform_transform_coefficient_must_be_complete() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x0a, 0x00, 0x01, 0x00, 0x00, 0x40]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("a uniform F2DOT14 coefficient occupies two bytes");

    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

#[test]
fn both_axis_scale_coefficients_must_be_complete() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x42, 0x00, 0x01, 0x00, 0x00, 0x40, 0x00, 0x20]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("the vertical scale cannot be a partial F2DOT14 value");

    assert_eq!(diagnostic.span(), ByteSpan::sized(18, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

#[test]
fn complete_matrix_requires_four_coefficients() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x82, 0x00, 0x01, 0x00, 0x00]);
    bytes.extend([0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("the final matrix coefficient cannot be truncated");

    assert_eq!(diagnostic.span(), ByteSpan::sized(22, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

#[test]
fn composite_instruction_length_must_be_complete() {
    let mut bytes = header(-1);
    bytes.extend([0x01, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("an instruction flag requires a complete length word");

    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

#[test]
fn composite_instruction_program_cannot_cross_the_span() {
    let mut bytes = header(-1);
    bytes.extend([0x01, 0x02, 0x00, 0x01, 0x00, 0x00]);
    bytes.extend([0x00, 0x02, 0xaa]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("a two-byte instruction program cannot contain one byte");

    assert_eq!(diagnostic.span(), ByteSpan::sized(18, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

#[test]
fn more_components_requires_another_complete_flag_word() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x22, 0x00, 0x01, 0x00, 0x00]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("MORE_COMPONENTS promises a following component record");

    assert_eq!(diagnostic.span(), Some(ByteSpan::empty_at(16)));
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 0,
        }
    );
}

#[test]
fn optional_composite_alignment_byte_must_be_zero() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x7f]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic =
        CompositeGlyph::decode(&bytes, span).expect_err("nonzero data is not an alignment byte");

    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 1));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::InvalidCompositeGlyphPadding { value: 0x7f })
    );
}

#[test]
fn at_most_one_composite_alignment_byte_is_permitted() {
    let mut bytes = header(-1);
    bytes.extend([0x00, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    let diagnostic = CompositeGlyph::decode(&bytes, span)
        .expect_err("two bytes after the final component are not alignment");

    assert_eq!(diagnostic.span(), ByteSpan::between(16, 18));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::UnexpectedCompositeGlyphData { length: 2 })
    );
}

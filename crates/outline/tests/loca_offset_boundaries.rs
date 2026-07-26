//! Offset-invariant coverage for TrueType glyph locations.

use glyphweave_outline::Loca;
use glyphweave_sfnt::{ByteSpan, Cause, LocaFormat, OutlineCause};

#[test]
fn first_offset_must_start_at_the_beginning_of_glyf() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&[0xaa; 4]);
    let table_span = ByteSpan::sized(0, 8).unwrap();
    let glyf_span = ByteSpan::sized(8, 4).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 1, LocaFormat::Long)
        .expect_err("leading glyf bytes are not assigned to glyph zero");

    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::NonzeroLocaStart { offset: 4 })
    );
}

#[test]
fn offsets_cannot_move_backward_between_glyphs() {
    let mut bytes = Vec::new();
    for offset in [0_u32, 8, 4] {
        bytes.extend_from_slice(&offset.to_be_bytes());
    }
    bytes.extend_from_slice(&[0xaa; 8]);
    let table_span = ByteSpan::sized(0, 12).unwrap();
    let glyf_span = ByteSpan::sized(12, 8).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Long)
        .expect_err("glyph locations must be nondecreasing");

    assert_eq!(diagnostic.span(), ByteSpan::sized(8, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::DecreasingLocaOffset {
            index: 2,
            offset: 4,
            previous: 8,
        })
    );
}

#[test]
fn long_offsets_must_be_word_aligned() {
    let mut bytes = Vec::new();
    for offset in [0_u32, 3, 4] {
        bytes.extend_from_slice(&offset.to_be_bytes());
    }
    bytes.extend_from_slice(&[0xaa; 4]);
    let table_span = ByteSpan::sized(0, 12).unwrap();
    let glyf_span = ByteSpan::sized(12, 4).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Long)
        .expect_err("every glyph offset must be word aligned");

    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::MisalignedLocaOffset {
            index: 1,
            offset: 3,
        })
    );
}

#[test]
fn intermediate_offset_cannot_escape_glyf_and_return() {
    let mut bytes = Vec::new();
    for offset in [0_u32, 12, 10] {
        bytes.extend_from_slice(&offset.to_be_bytes());
    }
    bytes.extend_from_slice(&[0xaa; 10]);
    let table_span = ByteSpan::sized(0, 12).unwrap();
    let glyf_span = ByteSpan::sized(12, 10).unwrap();

    let diagnostic = Loca::decode(&bytes, table_span, glyf_span, 2, LocaFormat::Long)
        .expect_err("every location must stay within glyf");

    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::LocaOffsetOutOfBounds {
            index: 1,
            offset: 12,
            glyf_length: 10,
        })
    );
}

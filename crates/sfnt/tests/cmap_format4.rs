//! Integration coverage for bounded `cmap` format 4 decoding.

use glyphweave_sfnt::{ByteSpan, Cause, CmapFormat4, GlyphId};

fn append_words(bytes: &mut Vec<u8>, words: &[u16]) {
    for word in words {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
}

fn minimal_words() -> Vec<u16> {
    vec![
        4, 24, 0, 2, 2, 0, 0,      // header
        0xffff, // endCode
        0,      // reservedPad
        0xffff, // startCode
        1,      // idDelta
        0,      // idRangeOffset
    ]
}

fn direct_delta_words() -> Vec<u16> {
    vec![
        4, 32, 7, 4, 4, 1, 0, // header
        0x0043, 0xffff, // endCode
        0,      // reservedPad
        0x0041, 0xffff, // startCode
        0xffc2, 1, // idDelta
        0, 0, // idRangeOffset
    ]
}

fn encode_words(words: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * 2);
    append_words(&mut bytes, words);
    bytes
}

fn decode_words(words: &[u16]) -> glyphweave_sfnt::Decoded<CmapFormat4> {
    CmapFormat4::decode(&encode_words(words), 0)
}

#[test]
fn direct_delta_segments_resolve_at_an_absolute_offset() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    append_words(&mut bytes, &direct_delta_words());

    let decoded = CmapFormat4::decode(&bytes, 3).expect("valid direct format-4 subtable");
    assert_eq!(decoded.span(), ByteSpan::sized(3, 32).unwrap());
    assert_eq!(decoded.language(), 7);
    assert_eq!(decoded.segment_count(), 2);
    assert_eq!(decoded.character_map().len(), 3);
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(3));
    assert_eq!(decoded.character_map().glyph_id('B'), GlyphId::new(4));
    assert_eq!(decoded.character_map().glyph_id('C'), GlyphId::new(5));
    assert_eq!(decoded.character_map().glyph_id('D'), GlyphId::NOTDEF);
}

#[test]
fn range_offset_segments_apply_delta_only_to_nonzero_glyph_words() {
    let mut bytes = vec![0xde, 0xad];
    append_words(
        &mut bytes,
        &[
            4, 38, 0, 4, 4, 1, 0, // header
            0x0043, 0xffff, // endCode
            0,      // reservedPad
            0x0041, 0xffff, // startCode
            2, 1, // idDelta
            4, 0, // idRangeOffset
            10, 0, 12, // glyphIdArray
        ],
    );

    let decoded = CmapFormat4::decode(&bytes, 2).expect("valid range format-4 subtable");
    assert_eq!(decoded.character_map().len(), 2);
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(12));
    assert_eq!(decoded.character_map().glyph_id('B'), GlyphId::NOTDEF);
    assert_eq!(decoded.character_map().glyph_id('C'), GlyphId::new(14));
}

#[test]
fn format_other_than_four_is_rejected_at_the_format_word() {
    let mut words = minimal_words();
    words[0] = 12;

    let diagnostic = decode_words(&words).expect_err("format 12 is not format 4");
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsupportedCmapFormat { found: 12 }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 2));
}

#[test]
fn declared_length_must_hold_the_minimum_sentinel_arrays() {
    let mut words = minimal_words();
    words[1] = 22;

    let diagnostic = decode_words(&words).expect_err("22 bytes cannot hold format 4");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapLength {
            declared: 22,
            minimum: 24,
            available: 24,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn declared_length_must_be_even() {
    let mut words = minimal_words();
    words[1] = 25;

    let diagnostic = decode_words(&words).expect_err("format-4 words require even length");
    assert_eq!(diagnostic.cause(), Cause::OddCmapLength { declared: 25 });
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn declared_length_cannot_exceed_available_bytes() {
    let mut bytes = encode_words(&minimal_words());
    bytes.pop();

    let diagnostic = CmapFormat4::decode(&bytes, 0).expect_err("last byte is truncated");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapLength {
            declared: 24,
            minimum: 24,
            available: 23,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn segment_count_encoding_must_be_positive_and_even() {
    for encoded in [0, 3] {
        let mut words = minimal_words();
        words[3] = encoded;

        let diagnostic = decode_words(&words).expect_err("invalid segCountX2");
        assert_eq!(
            diagnostic.cause(),
            Cause::InvalidCmapSegmentCount { encoded }
        );
        assert_eq!(diagnostic.span(), ByteSpan::sized(6, 2));
    }
}

#[test]
fn search_parameters_must_match_the_segment_count() {
    let mut words = minimal_words();
    words[4..7].copy_from_slice(&[4, 1, 2]);

    let diagnostic = decode_words(&words).expect_err("search parameters are not derived");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSearchParameters {
            expected: [2, 0, 0],
            actual: [4, 1, 2],
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(8, 6));
}

#[test]
fn declared_length_must_hold_every_segment_array() {
    let mut words = minimal_words();
    words[3..7].copy_from_slice(&[4, 4, 1, 0]);

    let diagnostic = decode_words(&words).expect_err("second segment does not fit");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapLength {
            declared: 24,
            minimum: 32,
            available: 24,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn reserved_padding_word_must_be_zero() {
    let mut words = minimal_words();
    words[8] = 1;

    let diagnostic = decode_words(&words).expect_err("reservedPad is nonzero");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapReservedPad { found: 1 }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 2));
}

#[test]
fn segment_start_cannot_exceed_its_end() {
    let mut words = direct_delta_words();
    words[10] = 0x0044;

    let diagnostic = decode_words(&words).expect_err("first segment is inverted");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSegment {
            index: 0,
            start: 0x0044,
            end: 0x0043,
            previous_end: None,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(14, 8));
}

#[test]
fn segment_ranges_cannot_overlap_their_predecessor() {
    let words = [
        4, 40, 0, 6, 4, 1, 2, // header
        0x0043, 0x0045, 0xffff, // endCode
        0,      // reservedPad
        0x0041, 0x0043, 0xffff, // startCode
        0, 0, 1, // idDelta
        0, 0, 0, // idRangeOffset
    ];

    let diagnostic = decode_words(&words).expect_err("second segment overlaps first");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSegment {
            index: 1,
            start: 0x0043,
            end: 0x0045,
            previous_end: Some(0x0043),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 10));
}

#[test]
fn final_segment_must_be_the_single_code_unit_sentinel() {
    let mut words = direct_delta_words();
    words[11] = 0xfffe;

    let diagnostic = decode_words(&words).expect_err("sentinel starts below 0xffff");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSentinelRange {
            start: 0xfffe,
            end: 0xffff,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 8));
}

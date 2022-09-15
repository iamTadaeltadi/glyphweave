//! Header-boundary coverage for `cmap` format 4 decoding.

#[path = "support/cmap_format4.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat4};
use support::{decode_words, encode_words, minimal_words};

#[test]
fn format_other_than_four_is_rejected_at_the_format_word() {
    let mut words = minimal_words();
    words[0] = 12;

    let diagnostic = decode_words(&words).expect_err("format 12 is not format 4");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::UnsupportedCmapFormat {
            expected: 4,
            found: 12,
        })
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
        Cause::Cmap(CmapCause::InvalidCmapLength {
            declared: 22,
            minimum: 24,
            available: 24,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn declared_length_must_be_even() {
    let mut words = minimal_words();
    words[1] = 25;

    let diagnostic = decode_words(&words).expect_err("format-4 words require even length");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::OddCmapLength { declared: 25 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn declared_length_cannot_exceed_available_bytes() {
    let mut bytes = encode_words(&minimal_words());
    bytes.pop();

    let diagnostic = CmapFormat4::decode(&bytes, 0).expect_err("last byte is truncated");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmapLength {
            declared: 24,
            minimum: 24,
            available: 23,
        })
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
            Cause::Cmap(CmapCause::InvalidCmapSegmentCount { encoded })
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
        Cause::Cmap(CmapCause::InvalidCmapSearchParameters {
            expected: [2, 0, 0],
            actual: [4, 1, 2],
        })
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
        Cause::Cmap(CmapCause::InvalidCmapLength {
            declared: 24,
            minimum: 32,
            available: 24,
        })
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
        Cause::Cmap(CmapCause::InvalidCmapReservedPad { found: 1 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 2));
}

#[test]
fn every_strict_prefix_of_a_valid_subtable_is_rejected() {
    let bytes = encode_words(&minimal_words());
    for length in 0..bytes.len() {
        assert!(
            CmapFormat4::decode(&bytes[..length], 0).is_err(),
            "prefix of {length} bytes unexpectedly decoded"
        );
    }
}

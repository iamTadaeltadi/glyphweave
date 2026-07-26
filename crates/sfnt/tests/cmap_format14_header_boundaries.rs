//! Header-boundary coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat14};
use support::{DefaultRange, NonDefaultMapping, encode_empty, encode_selector};

#[test]
fn format_other_than_fourteen_is_rejected_at_the_format_word() {
    let mut bytes = encode_empty();
    bytes[0..2].copy_from_slice(&12_u16.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("format 12 is not format 14");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::UnsupportedCmapFormat {
            expected: 14,
            found: 12,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 2));
}

#[test]
fn declared_length_must_hold_the_complete_header() {
    let mut bytes = encode_empty();
    bytes[2..6].copy_from_slice(&9_u32.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("header needs ten bytes");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14Length {
            declared: 9,
            minimum: 10,
            available: 10,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 4));
}

#[test]
fn declared_length_cannot_exceed_available_bytes() {
    let mut bytes = encode_empty();
    bytes.pop();

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("header is truncated");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14Length {
            declared: 10,
            minimum: 10,
            available: 9,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 4));
}

#[test]
fn selector_count_must_fit_the_declared_record_array() {
    let mut bytes = encode_empty();
    bytes[6..10].copy_from_slice(&1_u32.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("one selector needs 11 bytes");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::Cmap14SelectorCountOutOfBounds {
            count: 1,
            maximum: 0,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(6, 4));
}

#[test]
fn every_strict_prefix_of_a_valid_subtable_is_rejected() {
    let bytes = encode_selector(
        0xfe0f,
        &[DefaultRange {
            start: 0x2764,
            additional: 0,
        }],
        &[NonDefaultMapping {
            base: 0x82a6,
            glyph: 19,
        }],
    );
    for length in 0..bytes.len() {
        assert!(
            CmapFormat14::decode(&bytes[..length], 0).is_err(),
            "prefix of {length} bytes unexpectedly decoded"
        );
    }
}

#[test]
fn declared_span_excludes_neighboring_bytes() {
    let mut bytes = encode_empty();
    bytes.extend_from_slice(&[0xaa; 8]);

    let decoded = CmapFormat14::decode(&bytes, 0).expect("neighbor bytes are not declared");
    assert_eq!(decoded.span(), ByteSpan::sized(0, 10).unwrap());
    assert!(decoded.variation_selector_map().is_empty());
}

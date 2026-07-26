//! Header-boundary coverage for `cmap` format 12 decoding.

#[path = "support/cmap_format12.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat12};
use support::encode_groups;

#[test]
fn format_other_than_twelve_is_rejected_at_the_format_word() {
    let mut bytes = encode_groups(0, &[]);
    bytes[0..2].copy_from_slice(&4_u16.to_be_bytes());

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("format 4 is not format 12");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::UnsupportedCmapFormat {
            expected: 12,
            found: 4,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 2));
}

#[test]
fn reserved_word_must_be_zero() {
    let mut bytes = encode_groups(0, &[]);
    bytes[2..4].copy_from_slice(&7_u16.to_be_bytes());

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("reserved word is nonzero");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap12Reserved { found: 7 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(2, 2));
}

#[test]
fn declared_length_must_hold_the_complete_header() {
    let mut bytes = encode_groups(0, &[]);
    bytes[4..8].copy_from_slice(&12_u32.to_be_bytes());

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("header needs 16 bytes");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap12Length {
            declared: 12,
            minimum: 16,
            available: 16,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
}

#[test]
fn declared_length_cannot_exceed_available_bytes() {
    let mut bytes = encode_groups(0, &[(0x41, 0x41, 1)]);
    bytes.pop();

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("group is truncated");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap12Length {
            declared: 28,
            minimum: 16,
            available: 27,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
}

#[test]
fn declared_length_cannot_include_bytes_after_the_group_array() {
    let mut bytes = encode_groups(0, &[]);
    bytes.extend_from_slice(&[0; 4]);
    bytes[4..8].copy_from_slice(&20_u32.to_be_bytes());

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("four trailing bytes are declared");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::Cmap12LengthMismatch {
            declared: 20,
            expected: 16,
            groups: 0,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
}

#[test]
fn group_count_must_fit_the_declared_length() {
    let mut bytes = encode_groups(0, &[]);
    bytes[12..16].copy_from_slice(&1_u32.to_be_bytes());

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("one group needs 12 bytes");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::Cmap12LengthMismatch {
            declared: 16,
            expected: 28,
            groups: 1,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
}

#[test]
fn every_strict_prefix_of_a_valid_subtable_is_rejected() {
    let bytes = encode_groups(0, &[(0x1f980, 0x1f981, 70)]);
    for length in 0..bytes.len() {
        assert!(
            CmapFormat12::decode(&bytes[..length], 0).is_err(),
            "prefix of {length} bytes unexpectedly decoded"
        );
    }
}

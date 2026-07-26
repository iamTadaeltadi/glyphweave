//! Header-boundary coverage for `cmap` format 12 decoding.

#[path = "support/cmap_format12.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapFormat12};
use support::encode_groups;

#[test]
fn format_other_than_twelve_is_rejected_at_the_format_word() {
    let mut bytes = encode_groups(0, &[]);
    bytes[0..2].copy_from_slice(&4_u16.to_be_bytes());

    let diagnostic = CmapFormat12::decode(&bytes, 0).expect_err("format 4 is not format 12");
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsupportedCmapFormat {
            expected: 12,
            found: 4,
        }
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
        Cause::InvalidCmap12Reserved { found: 7 }
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
        Cause::InvalidCmap12Length {
            declared: 12,
            minimum: 16,
            available: 16,
        }
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
        Cause::InvalidCmap12Length {
            declared: 28,
            minimum: 16,
            available: 27,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(4, 4));
}

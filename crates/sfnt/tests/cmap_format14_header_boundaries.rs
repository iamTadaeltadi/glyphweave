//! Header-boundary coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat14};
use support::encode_empty;

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

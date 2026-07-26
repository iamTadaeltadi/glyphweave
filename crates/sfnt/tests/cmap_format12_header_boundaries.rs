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

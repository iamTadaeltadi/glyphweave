//! Group-boundary coverage for `cmap` format 12 decoding.

#[path = "support/cmap_format12.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause};
use support::decode_groups;

#[test]
fn group_start_cannot_exceed_its_end() {
    let diagnostic =
        decode_groups(0, &[(0x10020, 0x10010, 1)]).expect_err("sequential group is inverted");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmap12Group {
            index: 0,
            start: 0x10020,
            end: 0x10010,
            previous_end: None,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 12));
}

#[test]
fn groups_cannot_overlap_their_predecessor() {
    let diagnostic = decode_groups(0, &[(0x0041, 0x0043, 1), (0x0043, 0x0044, 9)])
        .expect_err("second group overlaps first");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmap12Group {
            index: 1,
            start: 0x0043,
            end: 0x0044,
            previous_end: Some(0x0043),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(28, 12));
}

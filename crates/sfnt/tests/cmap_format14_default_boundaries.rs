//! Default UVS range coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat14};
use support::{DefaultRange, encode_selector};

#[test]
fn default_ranges_cannot_overlap_their_predecessor() {
    let bytes = encode_selector(
        0xfe0f,
        &[
            DefaultRange {
                start: 0x4e00,
                additional: 2,
            },
            DefaultRange {
                start: 0x4e02,
                additional: 1,
            },
        ],
        &[],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("default ranges overlap");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14DefaultRange {
            selector: 0xfe0f,
            index: 1,
            start: 0x4e02,
            end: 0x4e03,
            previous_end: Some(0x4e02),
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(29, 4));
}

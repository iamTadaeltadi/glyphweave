//! Default UVS range coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{
    ByteSpan, Cause, CmapCause, CmapFormat14, VariationMapping, VariationSelector,
};
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

#[test]
fn adjacent_default_ranges_remain_separately_compressed() {
    let bytes = encode_selector(
        0xfe0f,
        &[
            DefaultRange {
                start: 0x4e00,
                additional: 2,
            },
            DefaultRange {
                start: 0x4e03,
                additional: 0,
            },
        ],
        &[],
    );

    let decoded = CmapFormat14::decode(&bytes, 0).expect("adjacent ranges do not overlap");
    let selector = VariationSelector::new('\u{fe0f}').unwrap();
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{4e02}', selector),
        Some(VariationMapping::Default)
    );
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{4e03}', selector),
        Some(VariationMapping::Default)
    );
    assert_eq!(decoded.variation_selector_map().default_range_count(), 2);
}

#[test]
fn default_range_cannot_enter_the_surrogate_area() {
    let bytes = encode_selector(
        0xfe0f,
        &[DefaultRange {
            start: 0xd7ff,
            additional: 1,
        }],
        &[],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("range ends at a surrogate");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14DefaultRange {
            selector: 0xfe0f,
            index: 0,
            start: 0xd7ff,
            end: 0xd800,
            previous_end: None,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(25, 4));
}

#[test]
fn default_range_start_cannot_exceed_unicode_maximum() {
    let bytes = encode_selector(
        0xfe0f,
        &[DefaultRange {
            start: 0x11_0000,
            additional: 0,
        }],
        &[],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("range starts above Unicode");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14DefaultRange {
            selector: 0xfe0f,
            index: 0,
            start: 0x11_0000,
            end: 0x11_0000,
            previous_end: None,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(25, 4));
}

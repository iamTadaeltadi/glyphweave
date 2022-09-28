//! Variation Selector record coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat14};
use support::{encode_selector, encode_selectors};

#[test]
fn ordinary_unicode_scalar_is_not_a_variation_selector() {
    let bytes = encode_selector(u32::from('A'), &[], &[]);

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("A is not a Variation Selector");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14VariationSelector {
            index: 0,
            selector: u32::from('A'),
            previous: None,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(10, 3));
}

#[test]
fn selector_value_cannot_exceed_the_unicode_domain() {
    let bytes = encode_selector(0x11_0000, &[], &[]);

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("selector is above Unicode");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14VariationSelector {
            index: 0,
            selector: 0x11_0000,
            previous: None,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(10, 3));
}

#[test]
fn selector_records_must_be_strictly_ordered() {
    let bytes = encode_selectors(&[0xe0100, 0xfe0f]);

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("selectors descend");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14VariationSelector {
            index: 1,
            selector: 0xfe0f,
            previous: Some(0xe0100),
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(21, 3));
}

#[test]
fn duplicate_selector_records_are_rejected() {
    let bytes = encode_selectors(&[0xfe0f, 0xfe0f]);

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("selector is duplicated");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14VariationSelector {
            index: 1,
            selector: 0xfe0f,
            previous: Some(0xfe0f),
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(21, 3));
}

#[test]
fn boundary_selectors_may_omit_both_mapping_tables() {
    let bytes = encode_selectors(&[0xfe00, 0xfe0f, 0xe0100, 0xe01ef]);

    let decoded = CmapFormat14::decode(&bytes, 0).expect("all selector boundaries are valid");
    assert_eq!(decoded.selector_record_count(), 4);
    assert!(decoded.variation_selector_map().is_empty());
}

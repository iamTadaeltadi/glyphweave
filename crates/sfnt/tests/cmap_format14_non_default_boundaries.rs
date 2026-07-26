//! Non-default UVS mapping coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{
    ByteSpan, Cause, CmapCause, CmapFormat14, GlyphId, VariationMapping, VariationSelector,
};
use support::{NonDefaultMapping, encode_selector};

#[test]
fn non_default_base_values_must_be_strictly_ordered() {
    let bytes = encode_selector(
        0xe0100,
        &[],
        &[
            NonDefaultMapping {
                base: 0x82a6,
                glyph: 7,
            },
            NonDefaultMapping {
                base: 0x82a6,
                glyph: 9,
            },
        ],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("base value is duplicated");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14NonDefaultMapping {
            selector: 0xe0100,
            index: 1,
            codepoint: 0x82a6,
            previous: Some(0x82a6),
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(34, 5));
}

#[test]
fn non_default_base_cannot_be_a_surrogate() {
    let bytes = encode_selector(
        0xe0100,
        &[],
        &[NonDefaultMapping {
            base: 0xd800,
            glyph: 7,
        }],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("surrogate is not scalar");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14NonDefaultMapping {
            selector: 0xe0100,
            index: 0,
            codepoint: 0xd800,
            previous: None,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(29, 5));
}

#[test]
fn non_default_base_cannot_exceed_unicode_maximum() {
    let bytes = encode_selector(
        0xe0100,
        &[],
        &[NonDefaultMapping {
            base: 0x11_0000,
            glyph: 7,
        }],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("base exceeds Unicode");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14NonDefaultMapping {
            selector: 0xe0100,
            index: 0,
            codepoint: 0x11_0000,
            previous: None,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(29, 5));
}

#[test]
fn explicit_notdef_mapping_remains_a_supported_sequence() {
    let bytes = encode_selector(
        0xe0100,
        &[],
        &[NonDefaultMapping {
            base: 0x82a6,
            glyph: 0,
        }],
    );

    let decoded = CmapFormat14::decode(&bytes, 0).expect("glyph zero is representable");
    let selector = VariationSelector::new('\u{e0100}').unwrap();
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{82a6}', selector),
        Some(VariationMapping::NonDefault(GlyphId::NOTDEF))
    );
    assert!(!decoded.variation_selector_map().is_empty());
}

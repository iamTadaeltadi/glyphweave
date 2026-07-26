//! Semantic coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{CharacterMap, CmapFormat14, GlyphId, VariationMapping, VariationSelector};
use support::{DefaultRange, NonDefaultMapping, encode_selector};

#[test]
fn default_and_non_default_sequences_resolve_distinctly() {
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
    let decoded = CmapFormat14::decode(&bytes, 0).expect("valid variation mappings");
    let selector = VariationSelector::new('\u{fe0f}').unwrap();
    let ordinary: CharacterMap = std::iter::once(('\u{2764}', GlyphId::new(7))).collect();

    assert_eq!(decoded.span().length(), bytes.len());
    assert_eq!(decoded.selector_record_count(), 1);
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{2764}', selector),
        Some(VariationMapping::Default)
    );
    assert_eq!(
        decoded
            .variation_selector_map()
            .glyph_id('\u{2764}', selector, &ordinary),
        Some(GlyphId::new(7))
    );
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{82a6}', selector),
        Some(VariationMapping::NonDefault(GlyphId::new(19)))
    );
}

#[test]
fn relative_targets_resolve_from_an_absolute_subtable_offset() {
    let subtable = encode_selector(
        0xe0100,
        &[DefaultRange {
            start: 0x4e00,
            additional: 1,
        }],
        &[],
    );
    let mut bytes = vec![0xaa; 13];
    let offset = bytes.len();
    bytes.extend_from_slice(&subtable);

    let decoded = CmapFormat14::decode(&bytes, offset).expect("relative target uses subtable base");
    let selector = VariationSelector::new('\u{e0100}').unwrap();
    assert_eq!(decoded.span().start(), offset);
    assert_eq!(decoded.span().length(), subtable.len());
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{4e01}', selector),
        Some(VariationMapping::Default)
    );
}

#[test]
fn selector_record_may_reference_only_default_sequences() {
    let mut bytes = encode_selector(
        0xfe0f,
        &[DefaultRange {
            start: 0x2764,
            additional: 0,
        }],
        &[],
    );
    bytes[17..21].copy_from_slice(&0_u32.to_be_bytes());

    let decoded = CmapFormat14::decode(&bytes, 0).expect("non-default offset is optional");
    let selector = VariationSelector::new('\u{fe0f}').unwrap();
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{2764}', selector),
        Some(VariationMapping::Default)
    );
    assert_eq!(
        decoded.variation_selector_map().non_default_mapping_count(),
        0
    );
}

#[test]
fn selector_record_may_reference_only_non_default_sequences() {
    let mut bytes = encode_selector(
        0xe0100,
        &[],
        &[NonDefaultMapping {
            base: 0x82a6,
            glyph: 19,
        }],
    );
    bytes[13..17].copy_from_slice(&0_u32.to_be_bytes());

    let decoded = CmapFormat14::decode(&bytes, 0).expect("default offset is optional");
    let selector = VariationSelector::new('\u{e0100}').unwrap();
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{82a6}', selector),
        Some(VariationMapping::NonDefault(GlyphId::new(19)))
    );
    assert_eq!(decoded.variation_selector_map().default_range_count(), 0);
}

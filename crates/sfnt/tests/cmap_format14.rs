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

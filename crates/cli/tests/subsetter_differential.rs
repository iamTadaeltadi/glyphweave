//! Live composite-closure comparisons with reference subsetters.

mod support;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::{GlyphId, VariationMapping, VariationSelector};

use support::{reference_font::glyf_from, reference_tool::subset};
use support::{
    reference_map::{format4_from, format12_from, format14_from},
    reference_tool::{subset_text, subset_unicode},
};

#[test]
fn fonttools_retains_the_glyphweave_component_closure() {
    let Some(bytes) = subset("pyftsubset", 5) else {
        return;
    };

    assert_reference_closure(&bytes);
}

#[test]
fn harfbuzz_retains_the_glyphweave_component_closure() {
    let Some(bytes) = subset("hb-subset", 5) else {
        return;
    };

    assert_reference_closure(&bytes);
}

#[test]
fn fonttools_preserves_the_selected_supplementary_mapping() {
    let Some(bytes) = subset_unicode("pyftsubset", 0x0001_f600) else {
        return;
    };

    assert_unicode_subset(&bytes);
}

#[test]
fn harfbuzz_preserves_the_selected_supplementary_mapping() {
    let Some(bytes) = subset_unicode("hb-subset", 0x0001_f600) else {
        return;
    };

    assert_unicode_subset(&bytes);
}

#[test]
fn fonttools_preserves_a_default_variation_sequence() {
    let Some(bytes) = subset_text("pyftsubset", "A\u{fe0f}") else {
        return;
    };

    assert_default_variation_sequence(&bytes);
}

#[test]
fn harfbuzz_preserves_a_default_variation_sequence() {
    let Some(bytes) = subset_text("hb-subset", "A\u{fe0f}") else {
        return;
    };

    assert_default_variation_sequence(&bytes);
}

fn assert_reference_closure(bytes: &[u8]) {
    let glyf = glyf_from(bytes);
    let initial: GlyphSet = std::iter::once(GlyphId::new(5)).collect();
    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(glyf.glyph_count(), 6);
    for raw in [0, 1] {
        assert!(glyf.outline(GlyphId::new(raw)).unwrap().is_empty());
    }
    for raw in [2, 3] {
        assert!(
            glyf.outline(GlyphId::new(raw))
                .unwrap()
                .as_simple()
                .is_some()
        );
    }
    for raw in [4, 5] {
        assert!(
            glyf.outline(GlyphId::new(raw))
                .unwrap()
                .as_composite()
                .is_some()
        );
    }
    assert_eq!(component_ids(&glyf, 4), [2, 3]);
    assert_eq!(component_ids(&glyf, 5), [4, 2]);
    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 2, 3, 4, 5]
    );
    assert_eq!(closure.maximum_component_depth(), 2);
}

fn component_ids(glyf: &glyphweave_outline::Glyf, raw: u16) -> Vec<u16> {
    glyf.outline(GlyphId::new(raw))
        .unwrap()
        .as_composite()
        .unwrap()
        .components()
        .iter()
        .map(|component| component.glyph_id().raw())
        .collect()
}

fn assert_unicode_subset(bytes: &[u8]) {
    let cmap = format12_from(bytes);

    assert_eq!(cmap.character_map().len(), 1);
    assert_eq!(cmap.character_map().glyph_id('\u{1f600}'), GlyphId::new(5));
    assert_eq!(cmap.character_map().glyph_id('A'), GlyphId::NOTDEF);
    assert_reference_closure(bytes);
}

fn assert_default_variation_sequence(bytes: &[u8]) {
    let ordinary = format4_from(bytes);
    let variation = format14_from(bytes);
    let selector = VariationSelector::new('\u{fe0f}').unwrap();

    assert_eq!(ordinary.character_map().glyph_id('A'), GlyphId::new(2));
    assert_eq!(
        variation.variation_selector_map().mapping('A', selector),
        Some(VariationMapping::Default)
    );
    assert_eq!(
        variation
            .variation_selector_map()
            .glyph_id('A', selector, ordinary.character_map()),
        Some(GlyphId::new(2))
    );
}

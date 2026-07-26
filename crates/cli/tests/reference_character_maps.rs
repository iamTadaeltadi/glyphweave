//! Semantic checks for the differential font's reviewed character maps.

mod support;

use glyphweave_sfnt::{GlyphId, VariationMapping, VariationSelector};

use support::reference_map::{format4, format12, format14};

#[test]
fn format_four_matches_the_fonttools_bmp_map() {
    let mappings: Vec<_> = format4()
        .character_map()
        .iter()
        .map(|(codepoint, glyph)| (u32::from(codepoint), glyph.raw()))
        .collect();

    assert_eq!(mappings, [(0x20, 1), (0x41, 2), (0x00c1, 4), (0x0301, 3)]);
    assert_eq!(format4().character_map().glyph_id('B'), GlyphId::NOTDEF);
}

#[test]
fn format_twelve_matches_the_fonttools_full_repertoire_map() {
    let mappings: Vec<_> = format12()
        .character_map()
        .iter()
        .map(|(codepoint, glyph)| (u32::from(codepoint), glyph.raw()))
        .collect();

    assert_eq!(
        mappings,
        [
            (0x20, 1),
            (0x41, 2),
            (0x00c1, 4),
            (0x0301, 3),
            (0x01f600, 5),
        ]
    );
}

#[test]
fn format_fourteen_matches_the_fonttools_default_sequence() {
    let selector = VariationSelector::new('\u{fe0f}').unwrap();
    let mappings = format14().variation_selector_map().clone();

    assert_eq!(
        mappings.mapping('A', selector),
        Some(VariationMapping::Default)
    );
    assert_eq!(
        mappings.glyph_id('A', selector, format12().character_map()),
        Some(GlyphId::new(2))
    );
}

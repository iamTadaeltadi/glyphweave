//! Semantic checks for the differential font's reviewed character maps.

mod support;

use glyphweave_sfnt::GlyphId;

use support::reference_map::format4;

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

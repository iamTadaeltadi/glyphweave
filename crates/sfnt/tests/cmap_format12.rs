//! Integration coverage for bounded `cmap` format 12 decoding.

#[path = "support/cmap_format12.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, CmapFormat12, GlyphId};
use support::encode_groups;

#[test]
fn sequential_groups_resolve_bmp_and_supplementary_mappings() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    bytes.extend_from_slice(&encode_groups(
        9,
        &[(0x0041, 0x0043, 3), (0x1f980, 0x1f981, 70)],
    ));

    let decoded = CmapFormat12::decode(&bytes, 3).expect("valid format-12 subtable");
    assert_eq!(decoded.span(), ByteSpan::sized(3, 40).unwrap());
    assert_eq!(decoded.language(), 9);
    assert_eq!(decoded.group_count(), 2);
    assert_eq!(decoded.character_map().len(), 5);
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(3));
    assert_eq!(decoded.character_map().glyph_id('C'), GlyphId::new(5));
    assert_eq!(decoded.character_map().glyph_id('🦀'), GlyphId::new(70));
    assert_eq!(decoded.character_map().glyph_id('🦁'), GlyphId::new(71));
}

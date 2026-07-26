//! Integration coverage for bounded `cmap` format 12 decoding.

use glyphweave_sfnt::{ByteSpan, CmapFormat12, GlyphId};

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

#[test]
fn sequential_groups_resolve_bmp_and_supplementary_mappings() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    push_u16(&mut bytes, 12);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, 40);
    push_u32(&mut bytes, 9);
    push_u32(&mut bytes, 2);
    for value in [0x0041, 0x0043, 3, 0x1f980, 0x1f981, 70] {
        push_u32(&mut bytes, value);
    }

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

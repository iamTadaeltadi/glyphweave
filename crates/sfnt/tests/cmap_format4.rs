//! Integration coverage for bounded `cmap` format 4 decoding.

#[path = "support/cmap_format4.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, CmapFormat4, GlyphId};
use support::{append_words, direct_delta_words, range_offset_words};

#[test]
fn direct_delta_segments_resolve_at_an_absolute_offset() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    append_words(&mut bytes, &direct_delta_words());

    let decoded = CmapFormat4::decode(&bytes, 3).expect("valid direct format-4 subtable");
    assert_eq!(decoded.span(), ByteSpan::sized(3, 32).unwrap());
    assert_eq!(decoded.language(), 7);
    assert_eq!(decoded.segment_count(), 2);
    assert_eq!(decoded.character_map().len(), 3);
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(3));
    assert_eq!(decoded.character_map().glyph_id('B'), GlyphId::new(4));
    assert_eq!(decoded.character_map().glyph_id('C'), GlyphId::new(5));
    assert_eq!(decoded.character_map().glyph_id('D'), GlyphId::NOTDEF);
}

#[test]
fn range_offset_segments_apply_delta_only_to_nonzero_glyph_words() {
    let mut bytes = vec![0xde, 0xad];
    append_words(&mut bytes, &range_offset_words());

    let decoded = CmapFormat4::decode(&bytes, 2).expect("valid range format-4 subtable");
    assert_eq!(decoded.character_map().len(), 2);
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(12));
    assert_eq!(decoded.character_map().glyph_id('B'), GlyphId::NOTDEF);
    assert_eq!(decoded.character_map().glyph_id('C'), GlyphId::new(14));
}

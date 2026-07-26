//! Integration coverage for bounded `cmap` format 4 decoding.

use glyphweave_sfnt::{ByteSpan, CmapFormat4, GlyphId};

fn append_words(bytes: &mut Vec<u8>, words: &[u16]) {
    for word in words {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
}

#[test]
fn direct_delta_segments_resolve_at_an_absolute_offset() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    append_words(
        &mut bytes,
        &[
            4, 32, 7, 4, 4, 1, 0, // header
            0x0043, 0xffff, // endCode
            0,      // reservedPad
            0x0041, 0xffff, // startCode
            0xffc2, 1, // idDelta
            0, 0, // idRangeOffset
        ],
    );

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

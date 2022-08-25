//! Integration coverage for bounded `cmap` format 4 decoding.

#[path = "support/cmap_format4.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, CmapFormat4, GlyphId};
use support::{
    append_words, decode_words, direct_delta_words, encode_words, minimal_words, range_offset_words,
};

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

#[test]
fn direct_delta_addition_wraps_modulo_sixteen_bits() {
    let mut words = direct_delta_words();
    words[7] = 0xfffe;
    words[10] = 0xfffe;
    words[12] = 3;

    let decoded = decode_words(&words).expect("wrapping delta subtable is valid");
    assert_eq!(
        decoded.character_map().glyph_id('\u{fffe}'),
        GlyphId::new(1)
    );
}

#[test]
fn sentinel_only_subtable_resolves_to_an_empty_character_map() {
    let decoded = decode_words(&minimal_words()).expect("minimal format-4 subtable is valid");
    assert_eq!(decoded.segment_count(), 1);
    assert!(decoded.character_map().is_empty());
}

#[test]
fn declared_span_excludes_neighboring_bytes() {
    let mut bytes = encode_words(&minimal_words());
    bytes.extend_from_slice(&[0xaa, 0xbb]);

    let decoded = CmapFormat4::decode(&bytes, 0).expect("neighbor bytes are not subtable data");
    assert_eq!(decoded.span(), ByteSpan::sized(0, 24).unwrap());
}

#[test]
fn each_range_offset_uses_the_address_of_its_own_word() {
    let words = [
        4, 44, 0, 6, 4, 1, 2, // header
        0x0041, 0x005a, 0xffff, // endCode
        0,      // reservedPad
        0x0041, 0x005a, 0xffff, // startCode
        1, 2, 1, // idDelta
        6, 6, 0, // idRangeOffset
        5, 7, // glyphIdArray
    ];

    let decoded = decode_words(&words).expect("two ranged segments are valid");
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(6));
    assert_eq!(decoded.character_map().glyph_id('Z'), GlyphId::new(9));
}

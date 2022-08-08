//! Integration coverage for bounded `cmap` format 4 decoding.

use glyphweave_sfnt::{ByteSpan, Cause, CmapFormat4, GlyphId};

fn append_words(bytes: &mut Vec<u8>, words: &[u16]) {
    for word in words {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
}

fn minimal_words() -> Vec<u16> {
    vec![
        4, 24, 0, 2, 2, 0, 0,      // header
        0xffff, // endCode
        0,      // reservedPad
        0xffff, // startCode
        1,      // idDelta
        0,      // idRangeOffset
    ]
}

fn decode_words(words: &[u16]) -> glyphweave_sfnt::Decoded<CmapFormat4> {
    let mut bytes = Vec::with_capacity(words.len() * 2);
    append_words(&mut bytes, words);
    CmapFormat4::decode(&bytes, 0)
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

#[test]
fn range_offset_segments_apply_delta_only_to_nonzero_glyph_words() {
    let mut bytes = vec![0xde, 0xad];
    append_words(
        &mut bytes,
        &[
            4, 38, 0, 4, 4, 1, 0, // header
            0x0043, 0xffff, // endCode
            0,      // reservedPad
            0x0041, 0xffff, // startCode
            2, 1, // idDelta
            4, 0, // idRangeOffset
            10, 0, 12, // glyphIdArray
        ],
    );

    let decoded = CmapFormat4::decode(&bytes, 2).expect("valid range format-4 subtable");
    assert_eq!(decoded.character_map().len(), 2);
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(12));
    assert_eq!(decoded.character_map().glyph_id('B'), GlyphId::NOTDEF);
    assert_eq!(decoded.character_map().glyph_id('C'), GlyphId::new(14));
}

#[test]
fn format_other_than_four_is_rejected_at_the_format_word() {
    let mut words = minimal_words();
    words[0] = 12;

    let diagnostic = decode_words(&words).expect_err("format 12 is not format 4");
    assert_eq!(
        diagnostic.cause(),
        Cause::UnsupportedCmapFormat { found: 12 }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(0, 2));
}

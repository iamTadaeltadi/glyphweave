//! Shared byte fixtures for format-4 integration suites.

use glyphweave_sfnt::CmapFormat4;

/// Appends unsigned words in the format's big-endian byte order.
pub fn append_words(bytes: &mut Vec<u8>, words: &[u16]) {
    for word in words {
        bytes.extend_from_slice(&word.to_be_bytes());
    }
}

/// Returns a valid sentinel-only subtable.
#[must_use]
pub fn minimal_words() -> Vec<u16> {
    vec![
        4, 24, 0, 2, 2, 0, 0,      // header
        0xffff, // endCode
        0,      // reservedPad
        0xffff, // startCode
        1,      // idDelta
        0,      // idRangeOffset
    ]
}

/// Returns a valid direct-delta subtable mapping `A` through `C`.
#[must_use]
pub fn direct_delta_words() -> Vec<u16> {
    vec![
        4, 32, 7, 4, 4, 1, 0, // header
        0x0043, 0xffff, // endCode
        0,      // reservedPad
        0x0041, 0xffff, // startCode
        0xffc2, 1, // idDelta
        0, 0, // idRangeOffset
    ]
}

/// Returns a valid glyph-array subtable with one missing entry.
#[must_use]
pub fn range_offset_words() -> Vec<u16> {
    vec![
        4, 38, 0, 4, 4, 1, 0, // header
        0x0043, 0xffff, // endCode
        0,      // reservedPad
        0x0041, 0xffff, // startCode
        2, 1, // idDelta
        4, 0, // idRangeOffset
        10, 0, 12, // glyphIdArray
    ]
}

/// Encodes unsigned words into a complete byte buffer.
#[must_use]
pub fn encode_words(words: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * 2);
    append_words(&mut bytes, words);
    bytes
}

/// Decodes a word fixture from byte offset zero.
///
/// # Errors
///
/// Returns the decoder's Diagnostic when the fixture is malformed.
pub fn decode_words(words: &[u16]) -> glyphweave_sfnt::Decoded<CmapFormat4> {
    CmapFormat4::decode(&encode_words(words), 0)
}

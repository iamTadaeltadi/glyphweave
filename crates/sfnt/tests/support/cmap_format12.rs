//! Shared byte fixtures for format-12 integration suites.

use glyphweave_sfnt::CmapFormat12;

/// Appends one unsigned 16-bit integer in big-endian order.
pub fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

/// Appends one unsigned 32-bit integer in big-endian order.
pub fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

/// Encodes a complete format-12 subtable from sequential groups.
///
/// # Panics
///
/// Panics only when an in-memory test fixture exceeds the format's 32-bit length or group count.
#[must_use]
pub fn encode_groups(language: u32, groups: &[(u32, u32, u32)]) -> Vec<u8> {
    let length = u32::try_from(16 + groups.len() * 12).expect("test fixture length fits u32");
    let mut bytes =
        Vec::with_capacity(usize::try_from(length).expect("u32 fits test target usize"));
    push_u16(&mut bytes, 12);
    push_u16(&mut bytes, 0);
    push_u32(&mut bytes, length);
    push_u32(&mut bytes, language);
    push_u32(
        &mut bytes,
        u32::try_from(groups.len()).expect("test fixture group count fits u32"),
    );
    for &(start, end, start_glyph) in groups {
        push_u32(&mut bytes, start);
        push_u32(&mut bytes, end);
        push_u32(&mut bytes, start_glyph);
    }
    bytes
}

/// Decodes a group fixture from byte offset zero.
///
/// # Errors
///
/// Returns the decoder's Diagnostic when the fixture is malformed.
pub fn decode_groups(
    language: u32,
    groups: &[(u32, u32, u32)],
) -> glyphweave_sfnt::Decoded<CmapFormat12> {
    CmapFormat12::decode(&encode_groups(language, groups), 0)
}

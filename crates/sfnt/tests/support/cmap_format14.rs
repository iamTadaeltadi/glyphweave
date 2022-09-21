//! Shared byte fixtures for format-14 integration suites.

/// A compressed default base-scalar range.
#[derive(Clone, Copy, Debug)]
pub struct DefaultRange {
    /// The first encoded base value.
    pub start: u32,
    /// The number of additional base values after `start`.
    pub additional: u8,
}

/// An explicit non-default base-scalar mapping.
#[derive(Clone, Copy, Debug)]
pub struct NonDefaultMapping {
    /// The encoded base value.
    pub base: u32,
    /// The explicit Glyph ID.
    pub glyph: u16,
}

/// Appends one unsigned 16-bit integer in big-endian order.
pub fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

/// Appends one unsigned 24-bit integer in big-endian order.
pub fn push_u24(bytes: &mut Vec<u8>, value: u32) {
    let encoded = value.to_be_bytes();
    bytes.extend_from_slice(&encoded[1..]);
}

/// Appends one unsigned 32-bit integer in big-endian order.
pub fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

/// Encodes one selector record with adjacent default and non-default target tables.
///
/// # Panics
///
/// Panics only when an in-memory test fixture exceeds the format's 32-bit length or counts.
#[must_use]
pub fn encode_selector(
    selector: u32,
    defaults: &[DefaultRange],
    non_defaults: &[NonDefaultMapping],
) -> Vec<u8> {
    let record_end = 21_u32;
    let default_length = 4 + u32::try_from(defaults.len()).expect("fixture count fits u32") * 4;
    let non_default_offset = record_end + default_length;
    let length = non_default_offset
        + 4
        + u32::try_from(non_defaults.len()).expect("fixture count fits u32") * 5;
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 14);
    push_u32(&mut bytes, length);
    push_u32(&mut bytes, 1);
    push_u24(&mut bytes, selector);
    push_u32(&mut bytes, record_end);
    push_u32(&mut bytes, non_default_offset);
    push_u32(
        &mut bytes,
        u32::try_from(defaults.len()).expect("fixture count fits u32"),
    );
    for range in defaults {
        push_u24(&mut bytes, range.start);
        bytes.push(range.additional);
    }
    push_u32(
        &mut bytes,
        u32::try_from(non_defaults.len()).expect("fixture count fits u32"),
    );
    for mapping in non_defaults {
        push_u24(&mut bytes, mapping.base);
        push_u16(&mut bytes, mapping.glyph);
    }
    bytes
}

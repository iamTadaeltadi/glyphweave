//! Bounded decoding for `AlternateSubst` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutTarget};

use super::AlternateSubstitution;
use crate::gsub::decode::decode_sequence_map;

impl AlternateSubstitution {
    /// Decodes an `AlternateSubst` subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid format, offsets, Coverage, or mapping-count mismatch.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        decode_sequence_map(bytes, span, 3, LayoutTarget::AlternateSet).map(Self::from_alternates)
    }
}

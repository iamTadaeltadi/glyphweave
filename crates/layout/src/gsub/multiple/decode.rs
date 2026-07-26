//! Bounded decoding for MultipleSubst format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutTarget};

use super::MultipleSubstitution;
use crate::gsub::decode::decode_sequence_map;

impl MultipleSubstitution {
    /// Decodes a MultipleSubst subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid format, offsets, Coverage, or mapping-count mismatch.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        decode_sequence_map(bytes, span, 2, LayoutTarget::SubstitutionSequence)
            .map(Self::from_mappings)
    }
}

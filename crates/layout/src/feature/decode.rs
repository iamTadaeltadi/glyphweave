//! Bounded Feature table decoding.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutTarget, Reader};

use super::Feature;
use crate::relative_target_span;

impl Feature {
    /// Decodes a Feature table at the beginning of `span`.
    ///
    /// Feature Parameters remain opaque, but their relative offset is checked and retained as an
    /// absolute byte position for the tag-specific pass that understands their representation.
    ///
    /// # Errors
    ///
    /// Returns a span-aware diagnostic for truncation or an invalid Feature Parameters target.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let parameters_field = reader.offset();
        let parameters_offset = reader.u16()?;
        let count = reader.u16()?;
        let mut lookup_indices = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            lookup_indices.push(reader.u16()?);
        }
        let parameters_position = if parameters_offset == 0 {
            None
        } else {
            Some(
                relative_target_span(
                    span,
                    parameters_field,
                    parameters_offset,
                    reader.offset() - span.start(),
                    LayoutTarget::FeatureParameters,
                )?
                .start(),
            )
        };
        Ok(Self::new(parameters_position, lookup_indices))
    }
}

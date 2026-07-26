//! Bounded Language System table decoding.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, Reader};

use super::LanguageSystem;
use crate::layout_error;

impl LanguageSystem {
    /// Decodes a Language System table at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic when `span` is outside `bytes`, a field is truncated, or
    /// the reserved lookup-order offset is nonzero.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let lookup_order_position = reader.offset();
        let lookup_order_offset = reader.u16()?;
        if lookup_order_offset != 0 {
            return Err(layout_error(
                lookup_order_position,
                LayoutCause::ReservedLookupOrderOffset {
                    offset: lookup_order_offset,
                },
            ));
        }

        let required = match reader.u16()? {
            u16::MAX => None,
            index => Some(index),
        };
        let count = reader.u16()?;
        let mut feature_indices = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            feature_indices.push(reader.u16()?);
        }
        Ok(Self::new(required, feature_indices))
    }
}

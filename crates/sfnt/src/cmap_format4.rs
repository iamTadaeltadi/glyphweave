//! Decoded Unicode BMP mappings from a `cmap` format 4 subtable.

mod decode;

use crate::{ByteSpan, CharacterMap, Decoded};

/// A validated and eagerly resolved `cmap` format 4 subtable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmapFormat4 {
    pub(crate) span: ByteSpan,
    pub(crate) language: u16,
    pub(crate) segment_count: u16,
    pub(crate) character_map: CharacterMap,
}

impl CmapFormat4 {
    /// Decodes and eagerly resolves a format-4 subtable at an absolute byte offset.
    ///
    /// # Errors
    ///
    /// Returns a span-aware decode Diagnostic when the header, segment arrays, sentinel, range
    /// offsets, or resolved Unicode mappings violate the format-4 invariants.
    pub fn decode(bytes: &[u8], offset: usize) -> Decoded<Self> {
        decode::decode(bytes, offset)
    }

    /// Returns the complete declared subtable Byte Span.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        self.span
    }

    /// Returns the encoded format-4 language field.
    #[must_use]
    pub const fn language(&self) -> u16 {
        self.language
    }

    /// Returns the number of encoded segments, including the sentinel.
    #[must_use]
    pub const fn segment_count(&self) -> u16 {
        self.segment_count
    }

    /// Returns the canonical resolved Unicode Character Map.
    #[must_use]
    pub const fn character_map(&self) -> &CharacterMap {
        &self.character_map
    }
}

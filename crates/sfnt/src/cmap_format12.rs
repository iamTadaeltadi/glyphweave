//! Decoded full-repertoire mappings from a `cmap` format 12 subtable.

mod decode;

use crate::{ByteSpan, CharacterMap, Decoded};

/// A validated and eagerly resolved `cmap` format 12 subtable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmapFormat12 {
    pub(crate) span: ByteSpan,
    pub(crate) language: u32,
    pub(crate) group_count: u32,
    pub(crate) character_map: CharacterMap,
}

impl CmapFormat12 {
    /// Decodes and eagerly resolves a format-12 subtable at an absolute byte offset.
    ///
    /// ```
    /// use glyphweave_sfnt::CmapFormat12;
    ///
    /// let empty = [
    ///     0, 12, // format
    ///     0, 0, // reserved
    ///     0, 0, 0, 16, // length
    ///     0, 0, 0, 0, // language
    ///     0, 0, 0, 0, // numGroups
    /// ];
    /// let decoded = CmapFormat12::decode(&empty, 0)?;
    /// assert!(decoded.character_map().is_empty());
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a span-aware decode Diagnostic when the header, group ordering, sequential Glyph
    /// IDs, or resolved Unicode mappings violate the format-12 invariants.
    pub fn decode(bytes: &[u8], offset: usize) -> Decoded<Self> {
        decode::decode(bytes, offset)
    }

    /// Returns the complete declared subtable Byte Span.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        self.span
    }

    /// Returns the encoded format-12 language field.
    #[must_use]
    pub const fn language(&self) -> u32 {
        self.language
    }

    /// Returns the number of encoded sequential groups.
    #[must_use]
    pub const fn group_count(&self) -> u32 {
        self.group_count
    }

    /// Returns the canonical resolved Unicode Character Map.
    #[must_use]
    pub const fn character_map(&self) -> &CharacterMap {
        &self.character_map
    }
}

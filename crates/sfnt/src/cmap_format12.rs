//! Decoded full-repertoire mappings from a `cmap` format 12 subtable.

use crate::{ByteSpan, CharacterMap};

/// A validated and eagerly resolved `cmap` format 12 subtable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmapFormat12 {
    pub(crate) span: ByteSpan,
    pub(crate) language: u32,
    pub(crate) group_count: u32,
    pub(crate) character_map: CharacterMap,
}

impl CmapFormat12 {
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

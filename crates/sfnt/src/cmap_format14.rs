//! Decoded Unicode variation sequences from a `cmap` format 14 subtable.

mod decode;

use crate::{ByteSpan, Decoded, VariationSelectorMap};

/// A validated and semantically resolved `cmap` format 14 subtable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmapFormat14 {
    pub(crate) span: ByteSpan,
    pub(crate) selector_record_count: u32,
    pub(crate) variation_selector_map: VariationSelectorMap,
}

impl CmapFormat14 {
    /// Decodes and resolves a format-14 subtable at an absolute byte offset.
    ///
    /// ```
    /// use glyphweave_sfnt::CmapFormat14;
    ///
    /// let empty = [
    ///     0, 14, // format
    ///     0, 0, 0, 10, // length
    ///     0, 0, 0, 0, // numVarSelectorRecords
    /// ];
    /// let decoded = CmapFormat14::decode(&empty, 0)?;
    /// assert!(decoded.variation_selector_map().is_empty());
    /// # Ok::<(), glyphweave_sfnt::Diagnostic>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a span-aware decode Diagnostic when the header, selector records, nested offsets,
    /// default ranges, explicit mappings, or target topology violates format-14 invariants.
    pub fn decode(bytes: &[u8], offset: usize) -> Decoded<Self> {
        decode::decode(bytes, offset)
    }

    /// Returns the complete declared subtable Byte Span.
    #[must_use]
    pub const fn span(&self) -> ByteSpan {
        self.span
    }

    /// Returns the number of encoded Variation Selector records.
    #[must_use]
    pub const fn selector_record_count(&self) -> u32 {
        self.selector_record_count
    }

    /// Returns the canonical resolved Variation Selector Map.
    #[must_use]
    pub const fn variation_selector_map(&self) -> &VariationSelectorMap {
        &self.variation_selector_map
    }
}

//! Decoded Unicode variation sequences from a `cmap` format 14 subtable.

use crate::{ByteSpan, VariationSelectorMap};

/// A validated and semantically resolved `cmap` format 14 subtable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CmapFormat14 {
    pub(crate) span: ByteSpan,
    pub(crate) selector_record_count: u32,
    pub(crate) variation_selector_map: VariationSelectorMap,
}

impl CmapFormat14 {
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

//! One canonical GSUB or GPOS Lookup.

use glyphweave_sfnt::{ByteSpan, LayoutTableKind};

use crate::LookupFlags;

mod extension;

/// An effective lookup type and its ordered payload subtables.
///
/// Extension wrappers are flattened during decoding, so `lookup_type` always names the real
/// substitution or positioning operation and each span locates its real payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lookup {
    table: LayoutTableKind,
    lookup_type: u16,
    flags: LookupFlags,
    subtables: Vec<ByteSpan>,
}

impl Lookup {
    /// Builds a Lookup without reordering or deduplicating its semantic subtable sequence.
    #[must_use]
    pub fn new(
        table: LayoutTableKind,
        lookup_type: u16,
        flags: LookupFlags,
        subtables: impl IntoIterator<Item = ByteSpan>,
    ) -> Self {
        Self {
            table,
            lookup_type,
            flags,
            subtables: subtables.into_iter().collect(),
        }
    }

    /// Returns the table family assigning meaning to the lookup type.
    #[must_use]
    pub const fn table(&self) -> LayoutTableKind {
        self.table
    }

    /// Returns the effective GSUB or GPOS lookup type.
    #[must_use]
    pub const fn lookup_type(&self) -> u16 {
        self.lookup_type
    }

    /// Returns the canonical lookup qualifiers.
    #[must_use]
    pub const fn flags(&self) -> LookupFlags {
        self.flags
    }

    /// Returns real payload spans in lookup subtable processing order.
    #[must_use]
    pub fn subtables(&self) -> &[ByteSpan] {
        &self.subtables
    }

    /// Returns the number of payload subtables.
    #[must_use]
    pub fn len(&self) -> usize {
        self.subtables.len()
    }

    /// Returns whether this model has no payload subtables.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.subtables.is_empty()
    }
}

#[cfg(test)]
mod tests;

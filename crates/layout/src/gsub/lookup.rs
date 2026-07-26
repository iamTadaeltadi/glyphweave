//! One semantic GSUB Lookup after extension flattening and subtable decoding.

use glyphweave_sfnt::{Decoded, LayoutCause, LayoutTableKind};

use crate::{GsubSubtable, Lookup, LookupFlags, layout_error};

/// Ordered same-typed GSUB subtables with their shared processing qualifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GsubLookup {
    lookup_type: u16,
    flags: LookupFlags,
    subtables: Vec<GsubSubtable>,
}

impl GsubLookup {
    pub(super) const fn from_parts(
        lookup_type: u16,
        flags: LookupFlags,
        subtables: Vec<GsubSubtable>,
    ) -> Self {
        Self {
            lookup_type,
            flags,
            subtables,
        }
    }

    /// Semantically decodes every real payload span of a shared Lookup.
    /// # Errors
    /// For a non-GSUB family or any malformed effective subtable.
    pub fn decode(bytes: &[u8], lookup: &Lookup) -> Decoded<Self> {
        if lookup.table() != LayoutTableKind::Substitution {
            let position = lookup.subtables().first().map_or(0, |span| span.start());
            return Err(layout_error(
                position,
                LayoutCause::UnexpectedLayoutTableKind {
                    expected: LayoutTableKind::Substitution,
                    actual: lookup.table(),
                },
            ));
        }
        let mut subtables = Vec::with_capacity(lookup.len());
        for &span in lookup.subtables() {
            subtables.push(GsubSubtable::decode(bytes, span, lookup.lookup_type())?);
        }
        Ok(Self {
            lookup_type: lookup.lookup_type(),
            flags: lookup.flags(),
            subtables,
        })
    }

    /// Returns the effective GSUB lookup type shared by every subtable.
    #[must_use]
    pub const fn lookup_type(&self) -> u16 {
        self.lookup_type
    }

    /// Returns the Lookup processing qualifiers.
    #[must_use]
    pub const fn flags(&self) -> LookupFlags {
        self.flags
    }

    /// Returns semantic subtables in shaping application order.
    #[must_use]
    pub fn subtables(&self) -> &[GsubSubtable] {
        &self.subtables
    }

    /// Returns the number of semantic subtables.
    #[must_use]
    pub fn len(&self) -> usize {
        self.subtables.len()
    }

    /// Returns whether no semantic subtable remains.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.subtables.is_empty()
    }
}

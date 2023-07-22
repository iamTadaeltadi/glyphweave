//! Canonical indexed OpenType LookupList model.

use glyphweave_sfnt::LayoutTableKind;

use crate::Lookup;

/// Lookups in the exact processing and index order defined by GSUB or GPOS.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LookupList {
    lookups: Vec<Lookup>,
}

impl LookupList {
    /// Builds a list without changing the order or identity of its Lookups.
    #[must_use]
    pub fn new(lookups: impl IntoIterator<Item = Lookup>) -> Self {
        Self {
            lookups: lookups.into_iter().collect(),
        }
    }

    /// Returns the Lookup at a zero-based LookupList index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&Lookup> {
        self.lookups.get(usize::from(index))
    }

    /// Returns all Lookups in processing order.
    #[must_use]
    pub fn lookups(&self) -> &[Lookup] {
        &self.lookups
    }

    /// Returns the common table family, or `None` for an empty list.
    #[must_use]
    pub fn table(&self) -> Option<LayoutTableKind> {
        self.lookups.first().map(Lookup::table)
    }

    /// Returns the number of indexed Lookups.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lookups.len()
    }

    /// Returns whether the list contains no Lookups.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }
}

#[cfg(test)]
mod tests;

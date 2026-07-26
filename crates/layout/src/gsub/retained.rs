//! GSUB semantics proven reachable and glyph-closed before remapping.

use crate::{GsubLookup, GsubLookupList};

/// A retained GSUB `LookupList` whose surviving glyph references belong to one closure.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RetainedGsub {
    pub(super) lookups: GsubLookupList,
}

impl RetainedGsub {
    /// Returns the retained Lookup at its original pre-pruning index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&GsubLookup> {
        self.lookups.lookup(index)
    }

    /// Returns the index-preserving retained `LookupList`.
    #[must_use]
    pub const fn lookup_list(&self) -> &GsubLookupList {
        &self.lookups
    }

    /// Returns the number of original `LookupList` slots, including empty placeholders.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lookups.len()
    }

    /// Returns whether the original `LookupList` had no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }
}

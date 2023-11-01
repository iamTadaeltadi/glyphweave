//! GSUB semantics whose glyph references are all in emitted index space.

use glyphweave_sfnt::NewGlyphId;

use crate::{GsubLookup, GsubLookupList};

/// A remapped GSUB `LookupList` that cannot contain input-space Glyph IDs.
///
/// ```
/// use glyphweave_layout::RemappedGsub;
///
/// let substitutions = RemappedGsub::default();
/// assert!(substitutions.is_empty());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RemappedGsub {
    pub(super) lookups: GsubLookupList<NewGlyphId>,
}

impl RemappedGsub {
    /// Returns one remapped Lookup at its stable pre-compaction index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&GsubLookup<NewGlyphId>> {
        self.lookups.lookup(index)
    }

    /// Returns the complete index-preserving remapped `LookupList`.
    #[must_use]
    pub const fn lookup_list(&self) -> &GsubLookupList<NewGlyphId> {
        &self.lookups
    }

    /// Returns the number of original Lookup slots, including empty placeholders.
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

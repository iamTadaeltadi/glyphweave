//! GPOS Lookups proven reachable from the retained feature set.

use glyphweave_sfnt::GlyphId;

use crate::{GposLookup, GposLookupList};

impl GposLookupList<GlyphId> {
    /// Retains only the Lookups a surviving feature still references.
    ///
    /// GPOS positioning never extends the glyph set, so a Lookup's reachability is decided entirely
    /// by the retained feature set: the Lookup at index `i` survives when `reachable(i)` holds.
    /// Unreachable Lookups become empty placeholders so every original slot stays put and
    /// contextual sequence-lookup indices remain valid until the cross-table index compaction in
    /// item 3.6. Glyph-level pruning of the surviving subtables is performed by
    /// [`GposLookupList::remap`], which drops entries whose glyphs leave the subset.
    #[must_use]
    pub fn retain(&self, mut reachable: impl FnMut(u16) -> bool) -> RetainedGpos {
        let lookups = self
            .lookups()
            .iter()
            .enumerate()
            .map(|(index, lookup)| {
                if u16::try_from(index).is_ok_and(&mut reachable) {
                    lookup.clone()
                } else {
                    GposLookup::from_parts(lookup.lookup_type(), lookup.flags(), Vec::new())
                }
            })
            .collect();
        RetainedGpos {
            lookups: Self::from_lookups(lookups),
        }
    }
}

/// A retained GPOS `LookupList` keeping only feature-reachable Lookups at their original indices.
///
/// ```
/// use glyphweave_layout::RetainedGpos;
///
/// let positioning = RetainedGpos::default();
/// assert!(positioning.is_empty());
/// assert!(positioning.lookup(0).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RetainedGpos {
    lookups: GposLookupList,
}

impl RetainedGpos {
    /// Returns the retained Lookup at its original pre-compaction index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&GposLookup> {
        self.lookups.lookup(index)
    }

    /// Returns the index-preserving retained `LookupList`.
    #[must_use]
    pub const fn lookup_list(&self) -> &GposLookupList {
        &self.lookups
    }

    /// Returns the number of original `LookupList` slots, including empty placeholders.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lookups.len()
    }

    /// Returns whether the original `LookupList` had no slots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }
}

#[cfg(test)]
mod tests;

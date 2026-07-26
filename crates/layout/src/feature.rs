//! Canonical OpenType Feature lookup selection.

mod decode;

/// One Feature table's optional parameter target and selected `LookupList` indices.
///
/// Lookup indices are a sorted set because lookup application follows `LookupList` order.
///
/// ```
/// use glyphweave_layout::Feature;
///
/// let feature = Feature::new(None, [6, 1, 6]);
/// assert_eq!(feature.lookup_indices(), &[1, 6]);
/// assert_eq!(feature.parameters_position(), None);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Feature {
    parameters_position: Option<usize>,
    lookup_indices: Vec<u16>,
}

impl Feature {
    /// Builds a feature and canonicalizes its lookup indices.
    #[must_use]
    pub fn new(
        parameters_position: Option<usize>,
        lookup_indices: impl IntoIterator<Item = u16>,
    ) -> Self {
        let mut lookup_indices: Vec<_> = lookup_indices.into_iter().collect();
        lookup_indices.sort_unstable();
        lookup_indices.dedup();
        Self {
            parameters_position,
            lookup_indices,
        }
    }

    /// Returns the checked absolute Feature Parameters position, when declared.
    #[must_use]
    pub const fn parameters_position(&self) -> Option<usize> {
        self.parameters_position
    }

    /// Returns sorted unique `LookupList` indices.
    #[must_use]
    pub fn lookup_indices(&self) -> &[u16] {
        &self.lookup_indices
    }

    /// Returns whether this feature has neither parameters nor referenced lookups.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.parameters_position.is_none() && self.lookup_indices.is_empty()
    }
}

#[cfg(test)]
mod tests;

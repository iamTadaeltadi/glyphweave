//! Deterministic Unicode scalar membership.

use std::collections::BTreeSet;

/// The unique Unicode scalar values requested for a subset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodepointSet {
    codepoints: BTreeSet<char>,
}

impl CodepointSet {
    /// Creates an empty Codepoint Set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            codepoints: BTreeSet::new(),
        }
    }

    /// Returns the number of requested Unicode scalar values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.codepoints.len()
    }

    /// Returns whether no Unicode scalar values are requested.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.codepoints.is_empty()
    }

    /// Returns whether `codepoint` is requested.
    #[must_use]
    pub fn contains(&self, codepoint: char) -> bool {
        self.codepoints.contains(&codepoint)
    }

    /// Visits requested Unicode scalar values in ascending numeric order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = char> + DoubleEndedIterator + '_ {
        self.codepoints.iter().copied()
    }
}

impl<'a> IntoIterator for &'a CodepointSet {
    type Item = char;
    type IntoIter = std::iter::Copied<std::collections::btree_set::Iter<'a, char>>;

    fn into_iter(self) -> Self::IntoIter {
        self.codepoints.iter().copied()
    }
}

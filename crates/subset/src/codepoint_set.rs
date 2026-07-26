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

    /// Requests a Unicode scalar value.
    ///
    /// Returns `true` when the value was newly inserted.
    pub fn insert(&mut self, codepoint: char) -> bool {
        self.codepoints.insert(codepoint)
    }

    /// Stops requesting a Unicode scalar value.
    ///
    /// Returns `true` when the value was present.
    pub fn remove(&mut self, codepoint: char) -> bool {
        self.codepoints.remove(&codepoint)
    }

    /// Retains only Unicode scalar values accepted by `predicate`.
    pub fn retain(&mut self, predicate: impl FnMut(&char) -> bool) {
        self.codepoints.retain(predicate);
    }

    /// Visits requested Unicode scalar values in ascending numeric order.
    #[must_use]
    pub fn iter(&self) -> impl ExactSizeIterator<Item = char> + DoubleEndedIterator + '_ {
        self.codepoints.iter().copied()
    }
}

impl Extend<char> for CodepointSet {
    fn extend<T: IntoIterator<Item = char>>(&mut self, codepoints: T) {
        self.codepoints.extend(codepoints);
    }
}

impl FromIterator<char> for CodepointSet {
    fn from_iter<T: IntoIterator<Item = char>>(codepoints: T) -> Self {
        Self {
            codepoints: codepoints.into_iter().collect(),
        }
    }
}

impl<'a> IntoIterator for &'a CodepointSet {
    type Item = char;
    type IntoIter = std::iter::Copied<std::collections::btree_set::Iter<'a, char>>;

    fn into_iter(self) -> Self::IntoIter {
        self.codepoints.iter().copied()
    }
}

#[cfg(test)]
mod tests;

//! One normalized contextual or chained-contextual action rule.

use super::SubstitutionRecord;

/// Ordered context predicates and nested Lookup actions with a nonempty input sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextRule<T> {
    backtrack: Vec<T>,
    input: Vec<T>,
    lookahead: Vec<T>,
    records: Vec<SubstitutionRecord>,
}

impl<T> ContextRule<T> {
    /// Creates a non-chained rule with a guaranteed first input predicate.
    #[must_use]
    pub fn new(
        first_input: T,
        remaining_input: impl IntoIterator<Item = T>,
        records: impl IntoIterator<Item = SubstitutionRecord>,
    ) -> Self {
        Self::chained([], first_input, remaining_input, [], records)
    }

    /// Creates a chained rule while preserving every encoded sequence direction and action order.
    #[must_use]
    pub fn chained(
        backtrack: impl IntoIterator<Item = T>,
        first_input: T,
        remaining_input: impl IntoIterator<Item = T>,
        lookahead: impl IntoIterator<Item = T>,
        records: impl IntoIterator<Item = SubstitutionRecord>,
    ) -> Self {
        let mut input = vec![first_input];
        input.extend(remaining_input);
        Self {
            backtrack: backtrack.into_iter().collect(),
            input,
            lookahead: lookahead.into_iter().collect(),
            records: records.into_iter().collect(),
        }
    }

    /// Returns backtrack predicates in encoded nearest-to-farthest order.
    #[must_use]
    pub fn backtrack(&self) -> &[T] {
        &self.backtrack
    }

    /// Returns the nonempty input predicate sequence.
    #[must_use]
    pub fn input(&self) -> &[T] {
        &self.input
    }

    /// Returns lookahead predicates in input sequence order.
    #[must_use]
    pub fn lookahead(&self) -> &[T] {
        &self.lookahead
    }

    /// Returns nested Lookup actions in application order.
    #[must_use]
    pub fn records(&self) -> &[SubstitutionRecord] {
        &self.records
    }
}

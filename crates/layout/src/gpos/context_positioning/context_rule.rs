//! One normalized contextual positioning action rule.

use super::ContextPositioningRecord;

/// Ordered context predicates and nested Lookup actions with a nonempty input sequence.
///
/// The predicate type `T` is an exact glyph, a class value, or a Coverage table
/// depending on the encoded `ContextPos` format. Each rule carries the GPOS
/// `SequenceLookupRecord`s that apply nested positioning Lookups to input positions.
///
/// # Examples
/// ```
/// use glyphweave_layout::{ContextPositioningRecord, ContextPositioningRule};
///
/// let rule = ContextPositioningRule::new(3_u16, [4, 5], [ContextPositioningRecord::new(1, 9)]);
/// assert!(rule.backtrack().is_empty());
/// assert_eq!(rule.input(), [3, 4, 5]);
/// assert_eq!(rule.records(), [ContextPositioningRecord::new(1, 9)]);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextPositioningRule<T> {
    backtrack: Vec<T>,
    input: Vec<T>,
    lookahead: Vec<T>,
    records: Vec<ContextPositioningRecord>,
}

impl<T> ContextPositioningRule<T> {
    /// Creates a non-chained rule with a guaranteed first input predicate.
    #[must_use]
    pub fn new(
        first_input: T,
        remaining_input: impl IntoIterator<Item = T>,
        records: impl IntoIterator<Item = ContextPositioningRecord>,
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
        records: impl IntoIterator<Item = ContextPositioningRecord>,
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
    pub fn records(&self) -> &[ContextPositioningRecord] {
        &self.records
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::{ContextPositioningRecord, ContextPositioningRule};

    #[test]
    fn non_chained_rule_keeps_a_nonempty_input_and_actions() {
        let rule = ContextPositioningRule::new(
            GlyphId::new(3),
            [GlyphId::new(4), GlyphId::new(5)],
            [ContextPositioningRecord::new(1, 9)],
        );
        assert!(rule.backtrack().is_empty());
        assert_eq!(
            rule.input(),
            [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5)]
        );
        assert!(rule.lookahead().is_empty());
        assert_eq!(rule.records(), [ContextPositioningRecord::new(1, 9)]);
    }

    #[test]
    fn chained_rule_preserves_all_three_sequence_directions() {
        let rule = ContextPositioningRule::chained(
            [2_u16, 1],
            3,
            [4, 5],
            [6, 7],
            [
                ContextPositioningRecord::new(2, 10),
                ContextPositioningRecord::new(0, 8),
            ],
        );
        assert_eq!(rule.backtrack(), [2, 1]);
        assert_eq!(rule.input(), [3, 4, 5]);
        assert_eq!(rule.lookahead(), [6, 7]);
        assert_eq!(
            rule.records(),
            [
                ContextPositioningRecord::new(2, 10),
                ContextPositioningRecord::new(0, 8),
            ]
        );
    }
}

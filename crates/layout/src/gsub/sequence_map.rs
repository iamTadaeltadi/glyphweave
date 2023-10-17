//! Shared canonical storage for semantically distinct glyph-sequence substitutions.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct SequenceMap<G> {
    mappings: BTreeMap<G, Vec<G>>,
}

impl<G: Copy + Ord> SequenceMap<G> {
    pub(super) fn new(mappings: impl IntoIterator<Item = (G, Vec<G>)>) -> Self {
        Self {
            mappings: mappings.into_iter().collect(),
        }
    }

    pub(super) fn get(&self, input: G) -> Option<&[G]> {
        self.mappings.get(&input).map(Vec::as_slice)
    }

    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = (G, &[G])> + '_ {
        self.mappings
            .iter()
            .map(|(&input, output)| (input, output.as_slice()))
    }

    pub(super) fn len(&self) -> usize {
        self.mappings.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

//! Shared canonical storage for semantically distinct glyph-sequence substitutions.

use std::collections::BTreeMap;

use glyphweave_sfnt::GlyphId;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct SequenceMap {
    mappings: BTreeMap<GlyphId, Vec<GlyphId>>,
}

impl SequenceMap {
    pub(super) fn new(mappings: impl IntoIterator<Item = (GlyphId, Vec<GlyphId>)>) -> Self {
        Self {
            mappings: mappings.into_iter().collect(),
        }
    }

    pub(super) fn get(&self, input: GlyphId) -> Option<&[GlyphId]> {
        self.mappings.get(&input).map(Vec::as_slice)
    }

    pub(super) fn iter(&self) -> impl ExactSizeIterator<Item = (GlyphId, &[GlyphId])> + '_ {
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

//! Immutable canonical subset selections and table decisions.

use std::collections::BTreeMap;

use glyphweave_outline::{GlyphSet, Remap};
use glyphweave_sfnt::{CharacterMap, FaceIndex, Tag};

use crate::{CodepointSet, Retention};

/// A validated immutable description of one deterministic subset operation.
///
/// ```
/// use glyphweave_sfnt::{CharacterMap, FaceIndex, GlyphId};
/// use glyphweave_subset::{CodepointSet, SubsetPlan};
///
/// let requested: CodepointSet = ['A'].into_iter().collect();
/// let map: CharacterMap = [('A', GlyphId::new(5))].into_iter().collect();
/// let plan = SubsetPlan::from_character_map(FaceIndex::FIRST, requested, &map, []);
///
/// assert!(plan.glyphs().contains(GlyphId::NOTDEF));
/// assert!(plan.glyphs().contains(GlyphId::new(5)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubsetPlan {
    face_index: FaceIndex,
    codepoints: CodepointSet,
    glyphs: GlyphSet,
    remap: Remap,
    retentions: BTreeMap<Tag, Retention>,
}

impl SubsetPlan {
    /// Constructs an initial plan by resolving requested scalars through `character_map`.
    ///
    /// The resulting Glyph Set contains every mapped request plus mandatory `.notdef`. Outline and
    /// layout closure can later supply an expanded final Glyph Set to [`Self::new`].
    #[must_use]
    pub fn from_character_map(
        face_index: FaceIndex,
        codepoints: CodepointSet,
        character_map: &CharacterMap,
        retentions: impl IntoIterator<Item = (Tag, Retention)>,
    ) -> Self {
        let glyphs = codepoints
            .iter()
            .map(|codepoint| character_map.glyph_id(codepoint))
            .collect();
        Self::new(face_index, codepoints, glyphs, retentions)
    }

    /// Constructs a plan and derives the only valid Remap for `glyphs`.
    ///
    /// Explicit `Drop` entries are canonicalized to absence because unspecified Tags also drop.
    #[must_use]
    pub fn new(
        face_index: FaceIndex,
        codepoints: CodepointSet,
        glyphs: GlyphSet,
        retentions: impl IntoIterator<Item = (Tag, Retention)>,
    ) -> Self {
        let remap = Remap::new(&glyphs);
        let mut canonical_retentions = BTreeMap::new();
        for (tag, retention) in retentions {
            if retention == Retention::Drop {
                canonical_retentions.remove(&tag);
            } else {
                canonical_retentions.insert(tag, retention);
            }
        }
        Self {
            face_index,
            codepoints,
            glyphs,
            remap,
            retentions: canonical_retentions,
        }
    }

    /// Returns the source Face Index selected by the plan.
    #[must_use]
    pub const fn face_index(&self) -> FaceIndex {
        self.face_index
    }

    /// Returns the requested Unicode scalar values.
    #[must_use]
    pub const fn codepoints(&self) -> &CodepointSet {
        &self.codepoints
    }

    /// Returns the final retained input Glyph Set.
    #[must_use]
    pub const fn glyphs(&self) -> &GlyphSet {
        &self.glyphs
    }

    /// Returns the canonical Remap derived from the retained Glyph Set.
    #[must_use]
    pub const fn remap(&self) -> &Remap {
        &self.remap
    }

    /// Returns the table decision for `tag`, defaulting to [`Retention::Drop`].
    #[must_use]
    pub fn retention(&self, tag: Tag) -> Retention {
        self.retentions.get(&tag).copied().unwrap_or_default()
    }

    /// Visits explicit retained-table decisions in ascending Tag order.
    #[must_use]
    pub fn retentions(
        &self,
    ) -> impl ExactSizeIterator<Item = (Tag, Retention)> + DoubleEndedIterator + '_ {
        self.retentions
            .iter()
            .map(|(&tag, &retention)| (tag, retention))
    }
}

#[cfg(test)]
mod tests;

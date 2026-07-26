//! Immutable canonical subset selections and table decisions.

use std::collections::BTreeMap;

use glyphweave_outline::{GlyphSet, Remap};
use glyphweave_sfnt::{FaceIndex, Tag};

use crate::{CodepointSet, Retention};

/// A validated immutable description of one deterministic subset operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubsetPlan {
    face_index: FaceIndex,
    codepoints: CodepointSet,
    glyphs: GlyphSet,
    remap: Remap,
    retentions: BTreeMap<Tag, Retention>,
}

impl SubsetPlan {
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
}

#[cfg(test)]
mod tests;

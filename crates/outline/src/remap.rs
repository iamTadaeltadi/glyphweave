//! Canonical dense remapping between input and emitted glyph spaces.

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::GlyphSet;

/// A total mapping from one Glyph Set into dense emitted glyph indices.
///
/// ```
/// use glyphweave_outline::{GlyphSet, Remap};
/// use glyphweave_sfnt::{GlyphId, NewGlyphId};
///
/// let glyphs: GlyphSet = [GlyphId::new(12), GlyphId::new(4)].into_iter().collect();
/// let remap = Remap::new(&glyphs);
///
/// assert_eq!(remap.new_glyph_id(GlyphId::new(4)), Some(NewGlyphId::new(1)));
/// assert_eq!(remap.glyph_id(NewGlyphId::new(2)), Some(GlyphId::new(12)));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Remap {
    forward: Vec<Option<NewGlyphId>>,
    reverse: Vec<GlyphId>,
}

impl Remap {
    /// Constructs the canonical dense Remap for `glyphs`.
    ///
    /// Ascending input Glyph IDs receive consecutive New Glyph IDs. Because every Glyph Set begins
    /// with `.notdef`, input glyph zero always maps to emitted glyph zero.
    #[must_use]
    pub fn new(glyphs: &GlyphSet) -> Self {
        let maximum = glyphs.iter().next_back().unwrap_or(GlyphId::NOTDEF);
        let mut forward = vec![None; usize::from(maximum.raw()) + 1];
        let mut reverse = Vec::with_capacity(glyphs.len());
        let mut raw = 0u16;
        for glyph in glyphs {
            let new_glyph = NewGlyphId::new(raw);
            forward[usize::from(glyph.raw())] = Some(new_glyph);
            reverse.push(glyph);
            raw = raw.saturating_add(1);
        }
        Self { forward, reverse }
    }

    /// Looks up the emitted New Glyph ID for a retained input Glyph ID.
    #[must_use]
    pub fn new_glyph_id(&self, glyph: GlyphId) -> Option<NewGlyphId> {
        self.forward
            .get(usize::from(glyph.raw()))
            .copied()
            .flatten()
    }

    /// Looks up the retained input Glyph ID for an emitted New Glyph ID.
    #[must_use]
    pub fn glyph_id(&self, new_glyph: NewGlyphId) -> Option<GlyphId> {
        self.reverse.get(usize::from(new_glyph.raw())).copied()
    }

    /// Visits `(GlyphId, NewGlyphId)` pairs in emitted Glyph ID order.
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = (GlyphId, NewGlyphId)> + DoubleEndedIterator + '_ {
        self.reverse
            .iter()
            .copied()
            .enumerate()
            .map(|(index, glyph)| {
                let raw = u16::try_from(index).unwrap_or(u16::MAX);
                (glyph, NewGlyphId::new(raw))
            })
    }

    /// Returns the number of mappings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.reverse.len()
    }

    /// Returns whether the Remap has no mappings.
    ///
    /// This is always `false` because its source Glyph Set always contains `.notdef`.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests;

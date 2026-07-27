//! GPOS mark-to-ligature attachment positioning (lookup type 5).

use glyphweave_sfnt::GlyphId;

use super::Anchor;
use crate::Coverage;

mod decode;
#[cfg(test)]
mod tests;

/// One covered mark glyph's mark class and its attachment anchor.
///
/// Every mark selected by the mark Coverage carries exactly one anchor and belongs to one mark
/// class strictly below the subtable's `markClassCount`. The class chooses which per-component
/// ligature anchor the mark attaches to.
///
/// ```
/// use glyphweave_layout::{Anchor, MarkLigatureRecord};
///
/// let record = MarkLigatureRecord::new(1, Anchor::coordinate(30, -10));
/// assert_eq!(record.class(), 1);
/// assert_eq!(record.anchor(), Anchor::coordinate(30, -10));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MarkLigatureRecord {
    class: u16,
    anchor: Anchor,
}

impl MarkLigatureRecord {
    /// Builds a mark record from its mark class and attachment anchor.
    #[must_use]
    pub const fn new(class: u16, anchor: Anchor) -> Self {
        Self { class, anchor }
    }

    /// Returns the mark class selecting which per-component anchor this mark attaches to.
    #[must_use]
    pub const fn class(self) -> u16 {
        self.class
    }

    /// Returns the mark's attachment anchor in glyph design space.
    #[must_use]
    pub const fn anchor(self) -> Anchor {
        self.anchor
    }
}

/// One covered ligature glyph's per-component grid of mark-class attachment anchors.
///
/// A ligature has one component per merged glyph. Each component supplies one optional anchor per
/// mark class, so the grid is `component_count` rows of `markClassCount` anchors. A `None` cell
/// means that component offers no attachment point for that mark class.
///
/// ```
/// use glyphweave_layout::{Anchor, LigatureAttachment};
///
/// let attach = LigatureAttachment::new([
///     vec![Some(Anchor::coordinate(100, 0)), None],
///     vec![None, Some(Anchor::coordinate(300, 0))],
/// ]);
/// assert_eq!(attach.component_count(), 2);
/// assert_eq!(attach.anchor(0, 0), Some(Anchor::coordinate(100, 0)));
/// assert_eq!(attach.anchor(0, 1), None);
/// assert_eq!(attach.anchor(1, 1), Some(Anchor::coordinate(300, 0)));
/// assert_eq!(attach.anchor(2, 0), None);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct LigatureAttachment {
    components: Vec<Vec<Option<Anchor>>>,
}

impl LigatureAttachment {
    /// Builds a ligature attachment from its component rows of optional mark-class anchors.
    #[must_use]
    pub fn new(components: impl IntoIterator<Item = Vec<Option<Anchor>>>) -> Self {
        Self {
            components: components.into_iter().collect(),
        }
    }

    /// Returns the component rows, each holding one optional anchor per mark class.
    #[must_use]
    pub fn components(&self) -> &[Vec<Option<Anchor>>] {
        &self.components
    }

    /// Returns the number of components in the ligature.
    #[must_use]
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Returns whether the ligature declares no components.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Resolves the anchor for one component and mark class, if the grid supplies one.
    ///
    /// Returns `None` when the component index is out of range, the mark class is out of range,
    /// or the cell is a null attachment point.
    #[must_use]
    pub fn anchor(&self, component: usize, mark_class: u16) -> Option<Anchor> {
        self.components
            .get(component)?
            .get(usize::from(mark_class))
            .copied()
            .flatten()
    }
}

/// Mark-to-ligature positioning: mark and ligature Coverages joined by a mark class grid.
///
/// A mark glyph found in the mark Coverage carries a [`MarkLigatureRecord`]; its mark class selects
/// a column in the [`LigatureAttachment`] of the ligature glyph it follows. Both record arrays run
/// parallel to their Coverage in Coverage Index order.
///
/// ```
/// use glyphweave_layout::{
///     Anchor, Coverage, LigatureAttachment, MarkLigaturePositioning, MarkLigatureRecord,
/// };
/// use glyphweave_sfnt::GlyphId;
///
/// let positioning = MarkLigaturePositioning::new(
///     Coverage::from_glyphs([GlyphId::new(20)]),
///     Coverage::from_glyphs([GlyphId::new(7)]),
///     1,
///     [MarkLigatureRecord::new(0, Anchor::coordinate(10, 40))],
///     [LigatureAttachment::new([vec![Some(Anchor::coordinate(120, 0))]])],
/// );
///
/// assert_eq!(positioning.mark_class_count(), 1);
/// let mark = positioning.mark(GlyphId::new(20)).unwrap();
/// assert_eq!(mark.anchor(), Anchor::coordinate(10, 40));
/// let ligature = positioning.ligature(GlyphId::new(7)).unwrap();
/// assert_eq!(ligature.anchor(0, mark.class()), Some(Anchor::coordinate(120, 0)));
/// assert!(positioning.ligature(GlyphId::new(8)).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MarkLigaturePositioning<G = GlyphId> {
    mark_coverage: Coverage<G>,
    ligature_coverage: Coverage<G>,
    mark_class_count: u16,
    marks: Vec<MarkLigatureRecord>,
    ligatures: Vec<LigatureAttachment>,
}

impl<G: Copy + Ord> MarkLigaturePositioning<G> {
    /// Builds mark-to-ligature positioning from its Coverages, class count, and record arrays.
    #[must_use]
    pub fn new(
        mark_coverage: Coverage<G>,
        ligature_coverage: Coverage<G>,
        mark_class_count: u16,
        marks: impl IntoIterator<Item = MarkLigatureRecord>,
        ligatures: impl IntoIterator<Item = LigatureAttachment>,
    ) -> Self {
        Self {
            mark_coverage,
            ligature_coverage,
            mark_class_count,
            marks: marks.into_iter().collect(),
            ligatures: ligatures.into_iter().collect(),
        }
    }

    /// Returns the Coverage selecting the mark glyphs.
    #[must_use]
    pub const fn mark_coverage(&self) -> &Coverage<G> {
        &self.mark_coverage
    }

    /// Returns the Coverage selecting the ligature glyphs.
    #[must_use]
    pub const fn ligature_coverage(&self) -> &Coverage<G> {
        &self.ligature_coverage
    }

    /// Returns the number of mark classes, one greater than the largest valid mark class.
    #[must_use]
    pub const fn mark_class_count(&self) -> u16 {
        self.mark_class_count
    }

    /// Returns the mark records in mark Coverage Index order.
    #[must_use]
    pub fn marks(&self) -> &[MarkLigatureRecord] {
        &self.marks
    }

    /// Returns the ligature attachments in ligature Coverage Index order.
    #[must_use]
    pub fn ligatures(&self) -> &[LigatureAttachment] {
        &self.ligatures
    }

    /// Returns the mark record for one glyph, or `None` when it is not a covered mark.
    #[must_use]
    pub fn mark(&self, glyph: G) -> Option<MarkLigatureRecord> {
        let index = self.mark_coverage.coverage_index(glyph)?;
        self.marks.get(usize::from(index)).copied()
    }

    /// Returns the ligature attachment for one glyph, or `None` when it is not a covered ligature.
    #[must_use]
    pub fn ligature(&self, glyph: G) -> Option<&LigatureAttachment> {
        let index = self.ligature_coverage.coverage_index(glyph)?;
        self.ligatures.get(usize::from(index))
    }

    /// Returns whether the subtable positions no marks against no ligatures.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.marks.is_empty() && self.ligatures.is_empty()
    }
}

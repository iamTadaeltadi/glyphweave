//! GPOS mark-to-mark attachment positioning (lookup type 6).

use glyphweave_sfnt::GlyphId;

use super::Anchor;
use crate::Coverage;

mod decode;
#[cfg(test)]
mod tests;

/// One mark-1 glyph's mark class and attachment anchor.
///
/// A mark-1 glyph is the mark being positioned against a preceding mark-2 glyph. Its mark class
/// selects which of the mark-2 glyph's per-class anchors it attaches to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mark1Record {
    class: u16,
    anchor: Anchor,
}

impl Mark1Record {
    /// Builds a mark-1 record from its mark class and attachment anchor.
    #[must_use]
    pub const fn new(class: u16, anchor: Anchor) -> Self {
        Self { class, anchor }
    }

    /// Returns the mark class selecting the mark-2 anchor this mark attaches to.
    #[must_use]
    pub const fn class(self) -> u16 {
        self.class
    }

    /// Returns the mark-1 glyph's own attachment anchor.
    #[must_use]
    pub const fn anchor(self) -> Anchor {
        self.anchor
    }
}

/// One mark-2 glyph's attachment anchors, one optional anchor per mark class.
///
/// The anchors run parallel to the mark classes, so a mark-1 glyph of class `n` attaches to the
/// anchor at index `n`. An entry is `None` when the encoded offset for that class was null.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Mark2Record {
    anchors: Vec<Option<Anchor>>,
}

impl Mark2Record {
    /// Builds a mark-2 record from its per-class optional anchors.
    #[must_use]
    pub fn new(anchors: impl IntoIterator<Item = Option<Anchor>>) -> Self {
        Self {
            anchors: anchors.into_iter().collect(),
        }
    }

    /// Returns the per-class optional anchors in mark-class order.
    #[must_use]
    pub fn anchors(&self) -> &[Option<Anchor>] {
        &self.anchors
    }

    /// Returns the anchor for one mark class, or `None` when absent or out of range.
    #[must_use]
    pub fn anchor(&self, class: u16) -> Option<Anchor> {
        self.anchors.get(usize::from(class)).copied().flatten()
    }

    /// Returns the number of mark-class anchor slots this record carries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.anchors.len()
    }

    /// Returns whether this record carries no mark-class anchor slot.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }
}

/// Mark-to-mark positioning: two coverages, a mark-class count, and parallel mark records.
///
/// The mark-1 records run parallel to the mark-1 Coverage in Coverage Index order, and the
/// mark-2 records run parallel to the mark-2 Coverage. Each mark-2 record holds one optional
/// anchor per mark class, so a covered mark-1 glyph attaches to the mark-2 anchor its class
/// selects.
///
/// ```
/// use glyphweave_layout::{Anchor, Coverage, Mark1Record, Mark2Record, MarkToMarkPositioning};
/// use glyphweave_sfnt::GlyphId;
///
/// let mark1_coverage = Coverage::from_glyphs([GlyphId::new(10)]);
/// let mark2_coverage = Coverage::from_glyphs([GlyphId::new(20)]);
/// let mark1 = [Mark1Record::new(0, Anchor::coordinate(5, 15))];
/// let mark2 = [Mark2Record::new([Some(Anchor::coordinate(30, 0))])];
/// let positioning = MarkToMarkPositioning::new(mark1_coverage, mark2_coverage, 1, mark1, mark2);
///
/// assert_eq!(positioning.mark_class_count(), 1);
/// let attaching = positioning.mark1_record(GlyphId::new(10)).unwrap();
/// assert_eq!(attaching.class(), 0);
/// assert_eq!(attaching.anchor(), Anchor::coordinate(5, 15));
/// let base = positioning.mark2_record(GlyphId::new(20)).unwrap();
/// assert_eq!(base.anchor(0), Some(Anchor::coordinate(30, 0)));
/// assert!(positioning.mark1_record(GlyphId::new(99)).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MarkToMarkPositioning<G = GlyphId> {
    mark1_coverage: Coverage<G>,
    mark2_coverage: Coverage<G>,
    mark_class_count: u16,
    mark1_records: Vec<Mark1Record>,
    mark2_records: Vec<Mark2Record>,
}

impl<G: Copy + Ord> MarkToMarkPositioning<G> {
    /// Builds mark-to-mark positioning from its coverages, class count, and parallel records.
    #[must_use]
    pub fn new(
        mark1_coverage: Coverage<G>,
        mark2_coverage: Coverage<G>,
        mark_class_count: u16,
        mark1_records: impl IntoIterator<Item = Mark1Record>,
        mark2_records: impl IntoIterator<Item = Mark2Record>,
    ) -> Self {
        Self {
            mark1_coverage,
            mark2_coverage,
            mark_class_count,
            mark1_records: mark1_records.into_iter().collect(),
            mark2_records: mark2_records.into_iter().collect(),
        }
    }

    /// Returns the Coverage selecting the mark-1 (attaching) glyphs.
    #[must_use]
    pub const fn mark1_coverage(&self) -> &Coverage<G> {
        &self.mark1_coverage
    }

    /// Returns the Coverage selecting the mark-2 (base mark) glyphs.
    #[must_use]
    pub const fn mark2_coverage(&self) -> &Coverage<G> {
        &self.mark2_coverage
    }

    /// Returns the number of mark classes shared by the two mark arrays.
    #[must_use]
    pub const fn mark_class_count(&self) -> u16 {
        self.mark_class_count
    }

    /// Returns the mark-1 records in mark-1 Coverage Index order.
    #[must_use]
    pub fn mark1_records(&self) -> &[Mark1Record] {
        &self.mark1_records
    }

    /// Returns the mark-2 records in mark-2 Coverage Index order.
    #[must_use]
    pub fn mark2_records(&self) -> &[Mark2Record] {
        &self.mark2_records
    }

    /// Returns the mark-1 record for one glyph, or `None` when it is not a mark-1 glyph.
    #[must_use]
    pub fn mark1_record(&self, glyph: G) -> Option<Mark1Record> {
        let index = self.mark1_coverage.coverage_index(glyph)?;
        self.mark1_records.get(usize::from(index)).copied()
    }

    /// Returns the mark-2 record for one glyph, or `None` when it is not a mark-2 glyph.
    #[must_use]
    pub fn mark2_record(&self, glyph: G) -> Option<&Mark2Record> {
        let index = self.mark2_coverage.coverage_index(glyph)?;
        self.mark2_records.get(usize::from(index))
    }
}

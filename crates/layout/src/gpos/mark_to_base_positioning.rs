//! GPOS mark-to-base attachment positioning (lookup type 4).

use glyphweave_sfnt::GlyphId;

use super::Anchor;
use crate::Coverage;

mod decode;
#[cfg(test)]
mod tests;

/// One covered mark glyph's class assignment and attachment anchor.
///
/// The class selects which of a base glyph's per-class anchors the mark attaches to, and the
/// anchor is the point on the mark that meets the base.
///
/// ```
/// use glyphweave_layout::{Anchor, MarkRecord};
///
/// let record = MarkRecord::new(1, Anchor::coordinate(30, 0));
/// assert_eq!(record.class(), 1);
/// assert_eq!(record.anchor(), Anchor::coordinate(30, 0));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MarkRecord {
    class: u16,
    anchor: Anchor,
}

impl MarkRecord {
    /// Builds a mark record from its class index and attachment anchor.
    #[must_use]
    pub const fn new(class: u16, anchor: Anchor) -> Self {
        Self { class, anchor }
    }

    /// Returns the mark class selecting the base anchor this mark attaches to.
    #[must_use]
    pub const fn class(self) -> u16 {
        self.class
    }

    /// Returns the anchor on the mark glyph that meets the base.
    #[must_use]
    pub const fn anchor(self) -> Anchor {
        self.anchor
    }
}

/// One covered base glyph's attachment anchors, one per mark class.
///
/// A base record supplies `markClassCount` anchors in class order. Any class may leave its slot
/// null, so each entry is optional.
///
/// ```
/// use glyphweave_layout::{Anchor, BaseRecord};
///
/// let record = BaseRecord::new([Some(Anchor::coordinate(200, 300)), None]);
/// assert_eq!(record.len(), 2);
/// assert_eq!(record.anchor(0), Some(Anchor::coordinate(200, 300)));
/// assert_eq!(record.anchor(1), None);
/// assert_eq!(record.anchor(2), None);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BaseRecord {
    anchors: Vec<Option<Anchor>>,
}

impl BaseRecord {
    /// Builds a base record from its per-class optional anchors, in mark-class order.
    #[must_use]
    pub fn new(anchors: impl IntoIterator<Item = Option<Anchor>>) -> Self {
        Self {
            anchors: anchors.into_iter().collect(),
        }
    }

    /// Returns the per-class anchors in mark-class order.
    #[must_use]
    pub fn anchors(&self) -> &[Option<Anchor>] {
        &self.anchors
    }

    /// Returns the base anchor for one mark class, or `None` when null or out of range.
    #[must_use]
    pub fn anchor(&self, mark_class: u16) -> Option<Anchor> {
        self.anchors.get(usize::from(mark_class)).copied().flatten()
    }

    /// Returns the number of mark classes this record supplies anchors for.
    #[must_use]
    pub fn len(&self) -> usize {
        self.anchors.len()
    }

    /// Returns whether this record supplies no per-class anchor.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }
}

/// The resolved attachment for one covered mark and base glyph pair.
///
/// The mark class comes from the mark record, the mark anchor is the point on the mark, and the
/// base anchor is the base's anchor for that class, which may be null.
///
/// ```
/// use glyphweave_layout::{Anchor, MarkBaseAttachment};
///
/// let attachment = MarkBaseAttachment::new(0, Anchor::coordinate(30, 0), None);
/// assert_eq!(attachment.mark_class(), 0);
/// assert_eq!(attachment.mark_anchor(), Anchor::coordinate(30, 0));
/// assert_eq!(attachment.base_anchor(), None);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MarkBaseAttachment {
    mark_class: u16,
    mark_anchor: Anchor,
    base_anchor: Option<Anchor>,
}

impl MarkBaseAttachment {
    /// Builds an attachment from a mark class, its mark anchor, and the resolved base anchor.
    #[must_use]
    pub const fn new(mark_class: u16, mark_anchor: Anchor, base_anchor: Option<Anchor>) -> Self {
        Self {
            mark_class,
            mark_anchor,
            base_anchor,
        }
    }

    /// Returns the mark class linking the mark to a column of base anchors.
    #[must_use]
    pub const fn mark_class(self) -> u16 {
        self.mark_class
    }

    /// Returns the anchor on the mark glyph.
    #[must_use]
    pub const fn mark_anchor(self) -> Anchor {
        self.mark_anchor
    }

    /// Returns the anchor on the base glyph for the mark's class, or `None` when null.
    #[must_use]
    pub const fn base_anchor(self) -> Option<Anchor> {
        self.base_anchor
    }
}

/// Mark-to-base positioning: mark and base Coverages paired with per-glyph attachment anchors.
///
/// Mark records run parallel to the mark Coverage and base records run parallel to the base
/// Coverage, each in Coverage Index order. A covered mark's class selects which of a covered
/// base's `markClassCount` anchors the pair attaches through.
///
/// ```
/// use glyphweave_layout::{Anchor, BaseRecord, Coverage, MarkRecord, MarkToBasePositioning};
/// use glyphweave_sfnt::GlyphId;
///
/// let marks = Coverage::from_glyphs([GlyphId::new(10), GlyphId::new(11)]);
/// let bases = Coverage::from_glyphs([GlyphId::new(20)]);
/// let mark_records = [
///     MarkRecord::new(0, Anchor::coordinate(30, 0)),
///     MarkRecord::new(1, Anchor::coordinate(-50, 10)),
/// ];
/// let base_records = [BaseRecord::new([Some(Anchor::coordinate(200, 300)), None])];
/// let positioning = MarkToBasePositioning::new(marks, bases, 2, mark_records, base_records);
///
/// let attachment = positioning.attachment(GlyphId::new(10), GlyphId::new(20)).unwrap();
/// assert_eq!(attachment.mark_class(), 0);
/// assert_eq!(attachment.mark_anchor(), Anchor::coordinate(30, 0));
/// assert_eq!(attachment.base_anchor(), Some(Anchor::coordinate(200, 300)));
///
/// let null_base = positioning.attachment(GlyphId::new(11), GlyphId::new(20)).unwrap();
/// assert_eq!(null_base.mark_class(), 1);
/// assert_eq!(null_base.base_anchor(), None);
///
/// assert!(positioning.attachment(GlyphId::new(12), GlyphId::new(20)).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MarkToBasePositioning<G = GlyphId> {
    mark_coverage: Coverage<G>,
    base_coverage: Coverage<G>,
    mark_class_count: u16,
    marks: Vec<MarkRecord>,
    bases: Vec<BaseRecord>,
}

impl<G: Copy + Ord> MarkToBasePositioning<G> {
    /// Builds mark-to-base positioning from its Coverages and parallel record arrays.
    #[must_use]
    pub fn new(
        mark_coverage: Coverage<G>,
        base_coverage: Coverage<G>,
        mark_class_count: u16,
        marks: impl IntoIterator<Item = MarkRecord>,
        bases: impl IntoIterator<Item = BaseRecord>,
    ) -> Self {
        Self {
            mark_coverage,
            base_coverage,
            mark_class_count,
            marks: marks.into_iter().collect(),
            bases: bases.into_iter().collect(),
        }
    }

    /// Returns the Coverage selecting the mark glyphs.
    #[must_use]
    pub const fn mark_coverage(&self) -> &Coverage<G> {
        &self.mark_coverage
    }

    /// Returns the Coverage selecting the base glyphs.
    #[must_use]
    pub const fn base_coverage(&self) -> &Coverage<G> {
        &self.base_coverage
    }

    /// Returns the number of distinct mark classes each base record supplies anchors for.
    #[must_use]
    pub const fn mark_class_count(&self) -> u16 {
        self.mark_class_count
    }

    /// Returns the mark records in mark-Coverage Index order.
    #[must_use]
    pub fn marks(&self) -> &[MarkRecord] {
        &self.marks
    }

    /// Returns the base records in base-Coverage Index order.
    #[must_use]
    pub fn bases(&self) -> &[BaseRecord] {
        &self.bases
    }

    /// Resolves the attachment for a mark and base glyph, or `None` when either is uncovered.
    ///
    /// The mark's class selects the base anchor; a null base anchor still resolves, reporting the
    /// mark class and mark anchor with a `None` base anchor.
    #[must_use]
    pub fn attachment(&self, mark: G, base: G) -> Option<MarkBaseAttachment> {
        let mark_index = usize::from(self.mark_coverage.coverage_index(mark)?);
        let base_index = usize::from(self.base_coverage.coverage_index(base)?);
        let mark_record = *self.marks.get(mark_index)?;
        let base_record = self.bases.get(base_index)?;
        Some(MarkBaseAttachment::new(
            mark_record.class(),
            mark_record.anchor(),
            base_record.anchor(mark_record.class()),
        ))
    }
}

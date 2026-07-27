//! GPOS cursive attachment positioning (lookup type 3).

use glyphweave_sfnt::GlyphId;

use super::Anchor;
use crate::Coverage;

mod decode;
#[cfg(test)]
mod tests;

/// One covered glyph's cursive entry and exit attachment anchors.
///
/// Cursive attachment joins a glyph's exit anchor to the following glyph's entry anchor. Either
/// anchor may be absent, so both are optional.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EntryExit {
    entry: Option<Anchor>,
    exit: Option<Anchor>,
}

impl EntryExit {
    /// Builds an entry/exit record from its optional anchors.
    #[must_use]
    pub const fn new(entry: Option<Anchor>, exit: Option<Anchor>) -> Self {
        Self { entry, exit }
    }

    /// Returns the entry anchor joining this glyph to its predecessor.
    #[must_use]
    pub const fn entry(self) -> Option<Anchor> {
        self.entry
    }

    /// Returns the exit anchor joining this glyph to its successor.
    #[must_use]
    pub const fn exit(self) -> Option<Anchor> {
        self.exit
    }

    /// Returns whether this record supplies neither an entry nor an exit anchor.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.entry.is_none() && self.exit.is_none()
    }
}

/// Cursive attachment positioning: a Coverage paired with per-glyph entry/exit anchors.
///
/// The entry/exit records run parallel to the Coverage in Coverage Index order, so a covered
/// glyph's record is found through its Coverage Index.
///
/// ```
/// use glyphweave_layout::{Anchor, CursivePositioning, Coverage, EntryExit};
/// use glyphweave_sfnt::GlyphId;
///
/// let coverage = Coverage::from_glyphs([GlyphId::new(7)]);
/// let records = [EntryExit::new(Some(Anchor::coordinate(50, 0)), None)];
/// let cursive = CursivePositioning::new(coverage, records);
///
/// assert_eq!(cursive.len(), 1);
/// let record = cursive.entry_exit(GlyphId::new(7)).unwrap();
/// assert_eq!(record.entry().unwrap().x(), 50);
/// assert!(record.exit().is_none());
/// assert!(cursive.entry_exit(GlyphId::new(8)).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CursivePositioning<G = GlyphId> {
    coverage: Coverage<G>,
    records: Vec<EntryExit>,
}

impl<G: Copy + Ord> CursivePositioning<G> {
    /// Builds cursive positioning from a Coverage and its parallel entry/exit records.
    #[must_use]
    pub fn new(coverage: Coverage<G>, records: impl IntoIterator<Item = EntryExit>) -> Self {
        Self {
            coverage,
            records: records.into_iter().collect(),
        }
    }

    /// Returns the Coverage selecting the cursively positioned glyphs.
    #[must_use]
    pub const fn coverage(&self) -> &Coverage<G> {
        &self.coverage
    }

    /// Returns the entry/exit records in Coverage Index order.
    #[must_use]
    pub fn records(&self) -> &[EntryExit] {
        &self.records
    }

    /// Returns the entry/exit record for one glyph, or `None` when it is not covered.
    #[must_use]
    pub fn entry_exit(&self, glyph: G) -> Option<EntryExit> {
        let index = self.coverage.coverage_index(glyph)?;
        self.records.get(usize::from(index)).copied()
    }

    /// Returns the number of entry/exit records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns whether the subtable positions no glyph.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

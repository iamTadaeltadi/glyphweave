//! GPOS pair positioning: explicit and class-based adjustments for adjacent glyph pairs.

use glyphweave_sfnt::GlyphId;

use crate::{ClassDefinition, Coverage, ValueRecord};

mod decode;

#[cfg(test)]
mod tests;

/// The paired adjustment a single `PairPos` entry applies to two adjacent glyphs.
///
/// The first record adjusts the leading glyph and the second record adjusts the trailing glyph.
/// A subtable carries an independent value format for each record, so either may be empty and
/// decode to an all-zero [`ValueRecord`].
///
/// ```
/// use glyphweave_layout::{PairAdjustment, ValueRecord};
///
/// let adjustment = PairAdjustment::new(ValueRecord::new(0, 0, -40, 0), ValueRecord::default());
/// assert_eq!(adjustment.first().x_advance(), -40);
/// assert!(adjustment.second().is_zero());
/// assert!(!adjustment.is_zero());
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PairAdjustment {
    first: ValueRecord,
    second: ValueRecord,
}

impl PairAdjustment {
    /// Builds a paired adjustment from its leading and trailing value records.
    #[must_use]
    pub const fn new(first: ValueRecord, second: ValueRecord) -> Self {
        Self { first, second }
    }

    /// The value record adjusting the leading glyph of the pair.
    #[must_use]
    pub const fn first(self) -> ValueRecord {
        self.first
    }

    /// The value record adjusting the trailing glyph of the pair.
    #[must_use]
    pub const fn second(self) -> ValueRecord {
        self.second
    }

    /// Returns whether neither glyph in the pair is adjusted.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.first.is_zero() && self.second.is_zero()
    }
}

/// One trailing glyph and the paired adjustment it receives after a fixed leading glyph.
///
/// `PairPos` format 1 groups these records into one `PairSet` per Coverage-selected leading glyph.
///
/// ```
/// use glyphweave_layout::{PairAdjustment, PairValue, ValueRecord};
/// use glyphweave_sfnt::GlyphId;
///
/// let value = PairValue::new(
///     GlyphId::new(7),
///     PairAdjustment::new(ValueRecord::new(0, 0, -15, 0), ValueRecord::default()),
/// );
/// assert_eq!(value.second(), GlyphId::new(7));
/// assert_eq!(value.adjustment().first().x_advance(), -15);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PairValue<G = GlyphId> {
    second: G,
    adjustment: PairAdjustment,
}

impl<G: Copy> PairValue<G> {
    /// Binds a trailing glyph to the paired adjustment applied after the leading glyph.
    #[must_use]
    pub const fn new(second: G, adjustment: PairAdjustment) -> Self {
        Self { second, adjustment }
    }

    /// The trailing glyph this record selects.
    #[must_use]
    pub const fn second(&self) -> G {
        self.second
    }

    /// The paired adjustment this record applies.
    #[must_use]
    pub const fn adjustment(&self) -> PairAdjustment {
        self.adjustment
    }
}

/// A decoded GPOS `PairPos` subtable in either its explicit-glyph or class-grid form.
///
/// ```
/// use glyphweave_layout::{Coverage, PairAdjustment, PairPositioning, PairValue, ValueRecord};
/// use glyphweave_sfnt::GlyphId;
///
/// let adjustment = PairAdjustment::new(ValueRecord::new(0, 0, -30, 0), ValueRecord::default());
/// let positioning = PairPositioning::Glyphs {
///     coverage: Coverage::from_glyphs([GlyphId::new(4)]),
///     sets: vec![vec![PairValue::new(GlyphId::new(7), adjustment)]],
/// };
/// assert_eq!(
///     positioning.adjustment(GlyphId::new(4), GlyphId::new(7)),
///     Some(adjustment),
/// );
/// assert_eq!(positioning.adjustment(GlyphId::new(4), GlyphId::new(8)), None);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PairPositioning<G = GlyphId> {
    /// Format 1: an explicit `PairSet` for each Coverage-selected leading glyph.
    Glyphs {
        /// The leading glyphs eligible for pair positioning, in Coverage order.
        coverage: Coverage<G>,
        /// One `PairSet` per covered leading glyph, positionally parallel to `coverage`.
        sets: Vec<Vec<PairValue<G>>>,
    },
    /// Format 2: a `class1Count` by `class2Count` grid of paired adjustments.
    Classes {
        /// The leading glyphs the subtable applies to.
        coverage: Coverage<G>,
        /// Class assignments resolving the leading glyph to a grid row.
        first_classes: ClassDefinition<G>,
        /// Class assignments resolving the trailing glyph to a grid column.
        second_classes: ClassDefinition<G>,
        /// Row-major grid indexed by first class then second class.
        grid: Vec<Vec<PairAdjustment>>,
    },
}

impl<G: Copy + Ord> PairPositioning<G> {
    /// Returns the Coverage selecting the leading glyphs this subtable positions.
    #[must_use]
    pub const fn coverage(&self) -> &Coverage<G> {
        match self {
            Self::Glyphs { coverage, .. } | Self::Classes { coverage, .. } => coverage,
        }
    }

    /// Resolves the paired adjustment for one leading and trailing glyph, if any applies.
    ///
    /// Returns `None` when the leading glyph is not covered or, in format 1, when no `PairSet`
    /// record names the trailing glyph. In format 2 any covered leading glyph with an in-range
    /// class pair resolves, including to an all-zero adjustment.
    #[must_use]
    pub fn adjustment(&self, first: G, second: G) -> Option<PairAdjustment> {
        let index = usize::from(self.coverage().coverage_index(first)?);
        match self {
            Self::Glyphs { sets, .. } => sets
                .get(index)
                .and_then(|set| set.iter().find(|value| value.second() == second))
                .map(PairValue::adjustment),
            Self::Classes {
                first_classes,
                second_classes,
                grid,
                ..
            } => {
                let row = usize::from(first_classes.class_of(first));
                let column = usize::from(second_classes.class_of(second));
                grid.get(row)?.get(column).copied()
            }
        }
    }
}

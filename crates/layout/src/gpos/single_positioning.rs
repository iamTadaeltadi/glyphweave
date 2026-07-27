//! Canonical single-glyph GPOS positioning adjustments.

use std::collections::BTreeMap;

use glyphweave_sfnt::GlyphId;

use crate::{ValueFormat, ValueRecord};

mod decode;

/// A Coverage-keyed map from one glyph to the value record positioning it.
///
/// A `SinglePos` subtable adjusts each covered glyph independently. Format 1 shares one value
/// record across every covered glyph; format 2 stores a parallel record for each. Both encode
/// their records against a single [`ValueFormat`], retained here so consumers can tell which
/// adjustments and device tables the subtable declared.
///
/// ```
/// use glyphweave_layout::{SinglePositioning, ValueFormat, ValueRecord};
/// use glyphweave_sfnt::GlyphId;
///
/// let format = ValueFormat::from_bits(0x0004).expect("x advance");
/// let positioning = SinglePositioning::new(
///     format,
///     [(GlyphId::new(7), ValueRecord::new(0, 0, -30, 0))],
/// );
/// assert_eq!(positioning.value_format(), format);
/// assert_eq!(
///     positioning.adjustment(GlyphId::new(7)),
///     Some(ValueRecord::new(0, 0, -30, 0)),
/// );
/// assert_eq!(positioning.adjustment(GlyphId::new(8)), None);
/// assert_eq!(positioning.len(), 1);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SinglePositioning<G = GlyphId> {
    value_format: ValueFormat,
    adjustments: BTreeMap<G, ValueRecord>,
}

impl<G: Copy + Ord> SinglePositioning<G> {
    /// Builds a positioning map; a later duplicate glyph replaces an earlier adjustment.
    #[must_use]
    pub fn new(
        value_format: ValueFormat,
        adjustments: impl IntoIterator<Item = (G, ValueRecord)>,
    ) -> Self {
        Self {
            value_format,
            adjustments: adjustments.into_iter().collect(),
        }
    }

    /// Returns the value format every retained record was decoded against.
    #[must_use]
    pub const fn value_format(&self) -> ValueFormat {
        self.value_format
    }

    /// Returns the positioning adjustment for one glyph.
    #[must_use]
    pub fn adjustment(&self, glyph: G) -> Option<ValueRecord> {
        self.adjustments.get(&glyph).copied()
    }

    /// Iterates adjustments in ascending Glyph ID order.
    #[must_use]
    pub fn adjustments(&self) -> impl ExactSizeIterator<Item = (G, ValueRecord)> + '_ {
        self.adjustments
            .iter()
            .map(|(&glyph, &record)| (glyph, record))
    }

    /// Returns the number of adjusted glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.adjustments.len()
    }

    /// Returns whether no glyph carries an adjustment.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adjustments.is_empty()
    }
}

#[cfg(test)]
mod tests;

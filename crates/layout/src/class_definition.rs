//! Canonical sparse OpenType glyph-class assignments.

use std::collections::BTreeMap;

use glyphweave_sfnt::GlyphId;

/// An ordered glyph-to-class mapping with implicit class zero.
///
/// Only nonzero assignments are stored. A glyph absent from the map always belongs to class zero.
///
/// ```
/// use glyphweave_layout::ClassDefinition;
/// use glyphweave_sfnt::GlyphId;
///
/// let classes = ClassDefinition::from_assignments([
///     (GlyphId::new(9), 2),
///     (GlyphId::new(3), 1),
/// ]);
/// assert_eq!(classes.class_of(GlyphId::new(3)), 1);
/// assert_eq!(classes.class_of(GlyphId::new(4)), 0);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClassDefinition {
    classes: BTreeMap<GlyphId, u16>,
}

impl ClassDefinition {
    /// Builds a canonical definition from assignments in arbitrary glyph order.
    ///
    /// Later assignments replace earlier assignments for the same glyph, and assigning class zero
    /// removes that glyph from the sparse representation.
    #[must_use]
    pub fn from_assignments(assignments: impl IntoIterator<Item = (GlyphId, u16)>) -> Self {
        let mut classes = BTreeMap::new();
        for (glyph, class) in assignments {
            if class == 0 {
                classes.remove(&glyph);
            } else {
                classes.insert(glyph, class);
            }
        }
        Self { classes }
    }

    /// Returns the assigned class, or implicit class zero when the glyph is absent.
    #[must_use]
    pub fn class_of(&self, glyph: GlyphId) -> u16 {
        self.classes.get(&glyph).copied().unwrap_or(0)
    }

    /// Iterates nonzero assignments in ascending Glyph ID order.
    #[must_use]
    pub fn assignments(&self) -> impl ExactSizeIterator<Item = (GlyphId, u16)> + '_ {
        self.classes.iter().map(|(&glyph, &class)| (glyph, class))
    }

    /// Returns the number of explicitly assigned glyphs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.classes.len()
    }

    /// Returns whether every glyph belongs to implicit class zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
}

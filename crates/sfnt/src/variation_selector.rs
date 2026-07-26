//! Unicode Variation Selectors as a closed scalar domain.

/// A Unicode scalar that can select a standardized or ideographic glyph variation.
///
/// The Unicode Variation Selector domain contains U+FE00 through U+FE0F and U+E0100 through
/// U+E01EF. Construction rejects every other scalar, so a [`VariationSelector`] can be used as a
/// map key without repeating range checks.
///
/// ```
/// use glyphweave_sfnt::VariationSelector;
///
/// let selector = VariationSelector::new('\u{fe0f}').unwrap();
/// assert_eq!(selector.value(), '\u{fe0f}');
/// assert!(VariationSelector::new('A').is_none());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariationSelector(char);

impl VariationSelector {
    /// Constructs a Variation Selector when `value` belongs to either Unicode selector range.
    #[must_use]
    pub const fn new(value: char) -> Option<Self> {
        match value as u32 {
            0xfe00..=0xfe0f | 0xe_0100..=0xe_01ef => Some(Self(value)),
            _ => None,
        }
    }

    /// Returns the Unicode scalar represented by this Variation Selector.
    #[must_use]
    pub const fn value(self) -> char {
        self.0
    }
}

impl From<VariationSelector> for char {
    fn from(selector: VariationSelector) -> Self {
        selector.value()
    }
}

#[cfg(test)]
mod tests {
    use super::VariationSelector;

    #[test]
    fn accepts_every_selector_range_boundary() {
        for scalar in ['\u{fe00}', '\u{fe0f}', '\u{e0100}', '\u{e01ef}'] {
            assert_eq!(
                VariationSelector::new(scalar).map(VariationSelector::value),
                Some(scalar)
            );
        }
    }

    #[test]
    fn rejects_scalars_outside_the_selector_ranges() {
        for scalar in ['\u{fdff}', '\u{fe10}', '\u{e00ff}', '\u{e01f0}', 'A'] {
            assert_eq!(VariationSelector::new(scalar), None);
        }
    }

    #[test]
    fn ordering_matches_unicode_scalar_order() {
        let standardized = VariationSelector::new('\u{fe0f}').unwrap();
        let ideographic = VariationSelector::new('\u{e0100}').unwrap();
        assert!(standardized < ideographic);
    }

    #[test]
    fn converts_back_into_its_unicode_scalar() {
        let selector = VariationSelector::new('\u{e0100}').unwrap();

        assert_eq!(char::from(selector), '\u{e0100}');
    }
}

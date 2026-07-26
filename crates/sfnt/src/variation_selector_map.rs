//! Compressed mappings for supported Unicode variation sequences.

use std::{collections::BTreeMap, ops::RangeInclusive};

use crate::{CharacterMap, GlyphId, VariationMapping, VariationSelector};

/// A canonical map from base scalars and Variation Selectors to glyph-selection rules.
///
/// Default mappings remain range-compressed. This keeps storage proportional to format-14 input
/// records while lookup still observes each supported variation sequence independently.
///
/// ```
/// use glyphweave_sfnt::{CharacterMap, VariationSelector, VariationSelectorMap};
///
/// let map = VariationSelectorMap::new();
/// let selector = VariationSelector::new('\u{fe0f}').unwrap();
/// assert_eq!(map.glyph_id('\u{2764}', selector, &CharacterMap::new()), None);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VariationSelectorMap {
    default_ranges: BTreeMap<(VariationSelector, char), char>,
    non_default_mappings: BTreeMap<(VariationSelector, char), GlyphId>,
}

impl VariationSelectorMap {
    /// Creates an empty Variation Selector Map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            default_ranges: BTreeMap::new(),
            non_default_mappings: BTreeMap::new(),
        }
    }

    /// Returns whether the map supports no variation sequences.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.default_ranges.is_empty() && self.non_default_mappings.is_empty()
    }

    /// Returns the number of compressed default ranges.
    #[must_use]
    pub fn default_range_count(&self) -> usize {
        self.default_ranges.len()
    }

    /// Returns the number of explicit non-default mappings.
    #[must_use]
    pub fn non_default_mapping_count(&self) -> usize {
        self.non_default_mappings.len()
    }

    /// Returns the encoded mapping kind for one variation sequence.
    #[must_use]
    pub fn mapping(&self, base: char, selector: VariationSelector) -> Option<VariationMapping> {
        if let Some(&glyph) = self.non_default_mappings.get(&(selector, base)) {
            return Some(VariationMapping::NonDefault(glyph));
        }
        let ((candidate_selector, _), end) =
            self.default_ranges.range(..=(selector, base)).next_back()?;
        (*candidate_selector == selector && base <= *end).then_some(VariationMapping::Default)
    }

    /// Resolves one variation sequence against the ordinary Character Map.
    ///
    /// Returns `None` when the sequence is unsupported. A supported default sequence can return
    /// `Some(GlyphId::NOTDEF)` when its base scalar is absent from `character_map`.
    #[must_use]
    pub fn glyph_id(
        &self,
        base: char,
        selector: VariationSelector,
        character_map: &CharacterMap,
    ) -> Option<GlyphId> {
        self.mapping(base, selector)
            .map(|mapping| mapping.glyph_id(base, character_map))
    }

    /// Visits compressed default ranges in selector and base-scalar order.
    #[must_use]
    pub fn default_ranges(
        &self,
    ) -> impl ExactSizeIterator<Item = (VariationSelector, RangeInclusive<char>)>
    + DoubleEndedIterator
    + '_ {
        self.default_ranges
            .iter()
            .map(|(&(selector, start), &end)| (selector, start..=end))
    }

    /// Visits explicit non-default mappings in selector and base-scalar order.
    #[must_use]
    pub fn non_default_mappings(
        &self,
    ) -> impl ExactSizeIterator<Item = (VariationSelector, char, GlyphId)> + DoubleEndedIterator + '_
    {
        self.non_default_mappings
            .iter()
            .map(|(&(selector, base), &glyph)| (selector, base, glyph))
    }

    pub(crate) fn insert_default_range(
        &mut self,
        selector: VariationSelector,
        start: char,
        end: char,
    ) {
        self.default_ranges.insert((selector, start), end);
    }

    pub(crate) fn insert_non_default(
        &mut self,
        selector: VariationSelector,
        base: char,
        glyph: GlyphId,
    ) {
        self.non_default_mappings.insert((selector, base), glyph);
    }
}

#[cfg(test)]
mod tests {
    use super::VariationSelectorMap;
    use crate::{CharacterMap, GlyphId, VariationMapping, VariationSelector};

    const fn selector(value: char) -> VariationSelector {
        VariationSelector::new(value).unwrap()
    }

    #[test]
    fn empty_map_resolves_every_sequence_as_unsupported() {
        let map = VariationSelectorMap::new();
        assert!(map.is_empty());
        assert_eq!(map.default_range_count(), 0);
        assert_eq!(map.non_default_mapping_count(), 0);
        assert_eq!(map.mapping('A', selector('\u{fe0f}')), None);
    }

    #[test]
    fn default_range_lookup_includes_both_boundaries() {
        let mut map = VariationSelectorMap::new();
        map.insert_default_range(selector('\u{fe0f}'), '\u{4e00}', '\u{4eff}');

        assert_eq!(
            map.mapping('\u{4e00}', selector('\u{fe0f}')),
            Some(VariationMapping::Default)
        );
        assert_eq!(
            map.mapping('\u{4eff}', selector('\u{fe0f}')),
            Some(VariationMapping::Default)
        );
        assert_eq!(map.mapping('\u{4dff}', selector('\u{fe0f}')), None);
        assert_eq!(map.mapping('\u{4f00}', selector('\u{fe0f}')), None);
        assert_eq!(map.default_range_count(), 1);
    }

    #[test]
    fn default_resolution_delegates_to_the_ordinary_map() {
        let mut map = VariationSelectorMap::new();
        map.insert_default_range(selector('\u{fe0f}'), 'A', 'B');
        let ordinary: CharacterMap = std::iter::once(('A', GlyphId::new(7))).collect();

        assert_eq!(
            map.glyph_id('A', selector('\u{fe0f}'), &ordinary),
            Some(GlyphId::new(7))
        );
        assert_eq!(
            map.glyph_id('B', selector('\u{fe0f}'), &ordinary),
            Some(GlyphId::NOTDEF)
        );
    }

    #[test]
    fn ranges_are_visited_in_selector_then_base_order() {
        let mut map = VariationSelectorMap::new();
        map.insert_default_range(selector('\u{e0100}'), 'Z', 'Z');
        map.insert_default_range(selector('\u{fe0f}'), 'M', 'P');
        map.insert_default_range(selector('\u{fe0f}'), 'A', 'C');

        let observed: Vec<_> = map.default_ranges().collect();
        assert_eq!(
            observed,
            vec![
                (selector('\u{fe0f}'), 'A'..='C'),
                (selector('\u{fe0f}'), 'M'..='P'),
                (selector('\u{e0100}'), 'Z'..='Z'),
            ]
        );
    }

    #[test]
    fn non_default_mapping_resolves_only_its_exact_sequence() {
        let mut map = VariationSelectorMap::new();
        map.insert_non_default(selector('\u{fe0f}'), '\u{2764}', GlyphId::new(19));
        let ordinary: CharacterMap = std::iter::once(('\u{2764}', GlyphId::new(7))).collect();

        assert_eq!(
            map.mapping('\u{2764}', selector('\u{fe0f}')),
            Some(VariationMapping::NonDefault(GlyphId::new(19)))
        );
        assert_eq!(
            map.glyph_id('\u{2764}', selector('\u{fe0f}'), &ordinary),
            Some(GlyphId::new(19))
        );
        assert_eq!(map.mapping('\u{2765}', selector('\u{fe0f}')), None);
        assert_eq!(map.mapping('\u{2764}', selector('\u{e0100}')), None);
    }

    #[test]
    fn explicit_notdef_mapping_is_not_confused_with_absence() {
        let mut map = VariationSelectorMap::new();
        map.insert_non_default(selector('\u{fe0f}'), 'A', GlyphId::NOTDEF);

        assert_eq!(
            map.mapping('A', selector('\u{fe0f}')),
            Some(VariationMapping::NonDefault(GlyphId::NOTDEF))
        );
        assert_eq!(
            map.glyph_id('A', selector('\u{fe0f}'), &CharacterMap::new()),
            Some(GlyphId::NOTDEF)
        );
        assert!(!map.is_empty());
    }

    #[test]
    fn non_default_mappings_are_visited_in_canonical_order() {
        let mut map = VariationSelectorMap::new();
        map.insert_non_default(selector('\u{e0100}'), 'A', GlyphId::new(3));
        map.insert_non_default(selector('\u{fe0f}'), 'Z', GlyphId::new(2));
        map.insert_non_default(selector('\u{fe0f}'), 'A', GlyphId::new(1));

        assert_eq!(
            map.non_default_mappings().collect::<Vec<_>>(),
            vec![
                (selector('\u{fe0f}'), 'A', GlyphId::new(1)),
                (selector('\u{fe0f}'), 'Z', GlyphId::new(2)),
                (selector('\u{e0100}'), 'A', GlyphId::new(3)),
            ]
        );
        assert_eq!(map.non_default_mapping_count(), 3);
    }
}

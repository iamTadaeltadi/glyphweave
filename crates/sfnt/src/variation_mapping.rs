//! Semantic outcomes encoded by Unicode variation sequences.

use crate::{CharacterMap, GlyphId};

/// The glyph-selection rule for one supported Unicode variation sequence.
///
/// A default mapping delegates to the ordinary Character Map for its base scalar. A non-default
/// mapping carries the exact Glyph ID from format 14, including an explicit `.notdef` value.
///
/// ```
/// use glyphweave_sfnt::{CharacterMap, GlyphId, VariationMapping};
///
/// let ordinary: CharacterMap = [('A', GlyphId::new(7))].into_iter().collect();
/// assert_eq!(VariationMapping::Default.glyph_id('A', &ordinary), GlyphId::new(7));
/// assert_eq!(
///     VariationMapping::NonDefault(GlyphId::new(19)).glyph_id('A', &ordinary),
///     GlyphId::new(19),
/// );
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VariationMapping {
    /// Use the ordinary Character Map entry for the sequence's base scalar.
    Default,
    /// Use this explicit format-14 Glyph ID instead of the ordinary mapping.
    NonDefault(GlyphId),
}

impl VariationMapping {
    /// Resolves this rule against the ordinary Character Map.
    #[must_use]
    pub fn glyph_id(self, base: char, character_map: &CharacterMap) -> GlyphId {
        match self {
            Self::Default => character_map.glyph_id(base),
            Self::NonDefault(glyph) => glyph,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VariationMapping;
    use crate::{CharacterMap, GlyphId};

    #[test]
    fn default_mapping_delegates_to_the_base_character() {
        let ordinary: CharacterMap = std::iter::once(('A', GlyphId::new(7))).collect();
        assert_eq!(
            VariationMapping::Default.glyph_id('A', &ordinary),
            GlyphId::new(7)
        );
        assert_eq!(
            VariationMapping::Default.glyph_id('B', &ordinary),
            GlyphId::NOTDEF
        );
    }

    #[test]
    fn non_default_mapping_ignores_the_base_character() {
        let ordinary: CharacterMap = std::iter::once(('A', GlyphId::new(7))).collect();
        assert_eq!(
            VariationMapping::NonDefault(GlyphId::new(19)).glyph_id('A', &ordinary),
            GlyphId::new(19)
        );
    }

    #[test]
    fn explicit_notdef_mapping_remains_supported() {
        let ordinary: CharacterMap = std::iter::once(('A', GlyphId::new(7))).collect();
        assert_eq!(
            VariationMapping::NonDefault(GlyphId::NOTDEF).glyph_id('A', &ordinary),
            GlyphId::NOTDEF
        );
    }
}

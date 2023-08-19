//! One semantic GSUB ligature rule.

use glyphweave_sfnt::GlyphId;

/// An ordered sequence of at least two components replaced by one ligature glyph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LigatureRule {
    components: Vec<GlyphId>,
    replacement: GlyphId,
}

impl LigatureRule {
    /// Creates a rule from its first two components, any remaining components, and output glyph.
    #[must_use]
    pub fn new(
        first: GlyphId,
        second: GlyphId,
        remaining: impl IntoIterator<Item = GlyphId>,
        replacement: GlyphId,
    ) -> Self {
        let mut components = vec![first, second];
        components.extend(remaining);
        Self {
            components,
            replacement,
        }
    }

    /// Returns every input component in matching order, including the first.
    #[must_use]
    pub fn components(&self) -> &[GlyphId] {
        &self.components
    }

    /// Returns the first component used to select the source LigatureSet.
    #[must_use]
    pub fn first(&self) -> GlyphId {
        self.components[0]
    }

    /// Returns the replacement ligature glyph.
    #[must_use]
    pub const fn replacement(&self) -> GlyphId {
        self.replacement
    }
}

#[cfg(test)]
mod tests {
    use glyphweave_sfnt::GlyphId;

    use super::LigatureRule;

    #[test]
    fn owns_a_nonempty_ordered_component_sequence() {
        let rule = LigatureRule::new(
            GlyphId::new(7),
            GlyphId::new(9),
            [GlyphId::new(7)],
            GlyphId::new(20),
        );
        assert_eq!(
            rule.components(),
            [GlyphId::new(7), GlyphId::new(9), GlyphId::new(7),]
        );
        assert_eq!(rule.first(), GlyphId::new(7));
        assert_eq!(rule.replacement(), GlyphId::new(20));
    }
}

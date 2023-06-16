//! Canonical OpenType language-system feature selection.

/// The required and optional `FeatureList` indices selected by one language system.
///
/// Optional indices are a sorted set because their source order does not control lookup
/// application order.
///
/// ```
/// use glyphweave_layout::LanguageSystem;
///
/// let language = LanguageSystem::new(Some(5), [8, 2, 8]);
/// assert_eq!(language.required_feature(), Some(5));
/// assert_eq!(language.feature_indices(), &[2, 8]);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LanguageSystem {
    required_feature: Option<u16>,
    feature_indices: Vec<u16>,
}

impl LanguageSystem {
    /// Builds a language system and canonicalizes optional feature indices.
    #[must_use]
    pub fn new(
        required_feature: Option<u16>,
        feature_indices: impl IntoIterator<Item = u16>,
    ) -> Self {
        let mut feature_indices: Vec<_> = feature_indices.into_iter().collect();
        feature_indices.sort_unstable();
        feature_indices.dedup();
        Self {
            required_feature,
            feature_indices,
        }
    }

    /// Returns the required `FeatureList` index, when one is declared.
    #[must_use]
    pub const fn required_feature(&self) -> Option<u16> {
        self.required_feature
    }

    /// Returns sorted unique optional `FeatureList` indices.
    #[must_use]
    pub fn feature_indices(&self) -> &[u16] {
        &self.feature_indices
    }

    /// Returns whether this language system selects no required or optional feature.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.required_feature.is_none() && self.feature_indices.is_empty()
    }
}

#[cfg(test)]
mod tests;

//! Canonical indexed OpenType `FeatureList` model.

use glyphweave_sfnt::Tag;

use crate::Feature;

/// Feature records in stable Tag order, including legal duplicate Tags.
///
/// Stable sorting preserves the relative order and therefore the index identity of equal-tagged
/// feature implementations.
///
/// ```
/// use glyphweave_layout::{Feature, FeatureList};
/// use glyphweave_sfnt::Tag;
///
/// let features = FeatureList::new([
///     (Tag::new(*b"liga"), Feature::default()),
///     (Tag::new(*b"kern"), Feature::default()),
/// ]);
/// assert_eq!(features.tag(0), Some(Tag::new(*b"kern")));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FeatureList {
    features: Vec<(Tag, Feature)>,
}

impl FeatureList {
    /// Builds a list using stable Tag ordering; repeated Tags remain distinct records.
    #[must_use]
    pub fn new(features: impl IntoIterator<Item = (Tag, Feature)>) -> Self {
        let mut features: Vec<_> = features.into_iter().collect();
        features.sort_by_key(|(tag, _)| *tag);
        Self { features }
    }

    /// Returns the Feature at a zero-based `FeatureList` index.
    #[must_use]
    pub fn feature(&self, index: u16) -> Option<&Feature> {
        self.features
            .get(usize::from(index))
            .map(|(_, feature)| feature)
    }

    /// Returns the Tag at a zero-based `FeatureList` index.
    #[must_use]
    pub fn tag(&self, index: u16) -> Option<Tag> {
        self.features.get(usize::from(index)).map(|(tag, _)| *tag)
    }

    /// Returns all indexed Feature records in stable Tag order.
    #[must_use]
    pub fn features(&self) -> &[(Tag, Feature)] {
        &self.features
    }

    /// Iterates every feature carrying the requested Tag.
    pub fn features_with_tag(&self, tag: Tag) -> impl Iterator<Item = (usize, &Feature)> {
        self.features
            .iter()
            .enumerate()
            .filter(move |(_, record)| record.0 == tag)
            .map(|(index, record)| (index, &record.1))
    }

    /// Returns the number of Feature records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Returns whether the list contains no Feature records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}

#[cfg(test)]
mod tests;

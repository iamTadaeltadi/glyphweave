//! Canonical OpenType Script language-system selection.

use glyphweave_sfnt::Tag;

use crate::LanguageSystem;

mod decode;

/// One Script table's default and tag-keyed language systems.
///
/// Language systems are held in sorted, unique Tag order because the OpenType Script table
/// requires its Language System Records to be sorted by Tag and free of duplicates.
///
/// ```
/// use glyphweave_layout::{LanguageSystem, Script};
/// use glyphweave_sfnt::Tag;
///
/// let script = Script::new(
///     Some(LanguageSystem::new(Some(0), [])),
///     [(Tag::new(*b"DEU "), LanguageSystem::new(None, [3, 1]))],
/// );
/// assert!(script.default_language().is_some());
/// assert_eq!(script.language(Tag::new(*b"DEU ")).unwrap().feature_indices(), &[1, 3]);
/// assert_eq!(script.language(Tag::new(*b"FRA ")), None);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Script {
    default_language: Option<LanguageSystem>,
    languages: Vec<(Tag, LanguageSystem)>,
}

impl Script {
    /// Builds a script and canonicalizes its language systems into sorted, unique Tag order.
    ///
    /// When a Tag repeats, the first language system provided for it is kept, matching the order
    /// a decoder observes Language System Records in.
    #[must_use]
    pub fn new(
        default_language: Option<LanguageSystem>,
        languages: impl IntoIterator<Item = (Tag, LanguageSystem)>,
    ) -> Self {
        let mut languages: Vec<_> = languages.into_iter().collect();
        languages.sort_by_key(|(tag, _)| *tag);
        languages.dedup_by_key(|(tag, _)| *tag);
        Self {
            default_language,
            languages,
        }
    }

    /// Returns the default language system applied when no Tag matches.
    #[must_use]
    pub const fn default_language(&self) -> Option<&LanguageSystem> {
        self.default_language.as_ref()
    }

    /// Returns the language system registered for `tag`, when present.
    #[must_use]
    pub fn language(&self, tag: Tag) -> Option<&LanguageSystem> {
        self.languages
            .binary_search_by_key(&tag, |(candidate, _)| *candidate)
            .ok()
            .map(|index| &self.languages[index].1)
    }

    /// Returns the tag-keyed language systems in sorted, unique Tag order.
    #[must_use]
    pub fn languages(&self) -> &[(Tag, LanguageSystem)] {
        &self.languages
    }

    /// Returns whether this script selects neither a default nor any tagged language system.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.default_language.is_none() && self.languages.is_empty()
    }
}

#[cfg(test)]
mod tests;

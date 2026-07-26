//! Canonical OpenType `ScriptList` model.

use glyphweave_sfnt::Tag;

use crate::Script;

mod decode;

/// The sorted unique Script records used by a GSUB or GPOS table.
///
/// ```
/// use glyphweave_layout::{Script, ScriptList};
/// use glyphweave_sfnt::Tag;
///
/// let scripts = ScriptList::new([
///     (Tag::new(*b"latn"), Script::default()),
///     (Tag::new(*b"arab"), Script::default()),
/// ]);
/// assert!(scripts.tags().eq([Tag::new(*b"arab"), Tag::new(*b"latn")]));
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScriptList {
    scripts: Vec<(Tag, Script)>,
}

impl ScriptList {
    /// Builds a list in sorted unique Tag order, keeping the first repeated Tag.
    #[must_use]
    pub fn new(scripts: impl IntoIterator<Item = (Tag, Script)>) -> Self {
        let mut scripts: Vec<_> = scripts.into_iter().collect();
        scripts.sort_by_key(|(tag, _)| *tag);
        scripts.dedup_by_key(|(tag, _)| *tag);
        Self { scripts }
    }

    /// Returns the Script registered for `tag`, when present.
    #[must_use]
    pub fn script(&self, tag: Tag) -> Option<&Script> {
        self.scripts
            .binary_search_by_key(&tag, |(candidate, _)| *candidate)
            .ok()
            .map(|index| &self.scripts[index].1)
    }

    /// Iterates Script tags in sorted order.
    #[must_use]
    pub fn tags(&self) -> impl ExactSizeIterator<Item = Tag> + '_ {
        self.scripts.iter().map(|(tag, _)| *tag)
    }

    /// Returns all tag-keyed Script records in sorted order.
    #[must_use]
    pub fn scripts(&self) -> &[(Tag, Script)] {
        &self.scripts
    }

    /// Returns the number of Script records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.scripts.len()
    }

    /// Returns whether the list contains no Script records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.scripts.is_empty()
    }
}

#[cfg(test)]
mod tests;

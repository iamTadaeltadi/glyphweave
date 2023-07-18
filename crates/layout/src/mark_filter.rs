//! Canonical mark-glyph filtering semantics for a Lookup.

/// The effective mark filter after applying OpenType lookup-flag precedence.
///
/// `IgnoreAll` dominates a filtering set, which dominates an attachment class.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum MarkFilter {
    /// Process marks without an additional mark-specific filter.
    #[default]
    None,
    /// Process marks in one GDEF mark attachment class.
    AttachmentClass(u8),
    /// Process marks in one GDEF mark glyph set.
    Set(u16),
    /// Ignore every mark glyph.
    IgnoreAll,
}

impl MarkFilter {
    /// Resolves raw lookup declarations using specification precedence.
    #[must_use]
    pub const fn resolve(
        ignore_marks: bool,
        filtering_set: Option<u16>,
        attachment_class: Option<u8>,
    ) -> Self {
        if ignore_marks {
            Self::IgnoreAll
        } else if let Some(set) = filtering_set {
            Self::Set(set)
        } else if let Some(class) = attachment_class {
            Self::AttachmentClass(class)
        } else {
            Self::None
        }
    }
}

#[cfg(test)]
mod tests;

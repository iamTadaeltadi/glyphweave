//! Canonical qualifiers shared by GSUB and GPOS Lookups.

use crate::MarkFilter;

/// Glyph-sequence qualifiers applied while matching one Lookup.
///
/// ```
/// use glyphweave_layout::{LookupFlags, MarkFilter};
///
/// let flags = LookupFlags::new(true, false, true, MarkFilter::Set(3));
/// assert!(flags.right_to_left());
/// assert_eq!(flags.mark_filter(), MarkFilter::Set(3));
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LookupFlags {
    right_to_left: bool,
    ignore_base_glyphs: bool,
    ignore_ligatures: bool,
    mark_filter: MarkFilter,
}

impl LookupFlags {
    /// Builds canonical lookup qualifiers with an already resolved mark filter.
    #[must_use]
    pub const fn new(
        right_to_left: bool,
        ignore_base_glyphs: bool,
        ignore_ligatures: bool,
        mark_filter: MarkFilter,
    ) -> Self {
        Self {
            right_to_left,
            ignore_base_glyphs,
            ignore_ligatures,
            mark_filter,
        }
    }

    /// Returns whether cursive positioning uses the final glyph on the baseline.
    #[must_use]
    pub const fn right_to_left(self) -> bool {
        self.right_to_left
    }

    /// Returns whether base glyphs are skipped while matching.
    #[must_use]
    pub const fn ignores_base_glyphs(self) -> bool {
        self.ignore_base_glyphs
    }

    /// Returns whether ligature glyphs are skipped while matching.
    #[must_use]
    pub const fn ignores_ligatures(self) -> bool {
        self.ignore_ligatures
    }

    /// Returns the effective mark-glyph filter after precedence resolution.
    #[must_use]
    pub const fn mark_filter(self) -> MarkFilter {
        self.mark_filter
    }
}

#[cfg(test)]
mod tests;

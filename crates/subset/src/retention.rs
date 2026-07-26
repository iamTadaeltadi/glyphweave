//! Explicit table and record retention decisions.

/// The action a subset pass takes for a table or record.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Retention {
    /// Omit the original structure and its bytes.
    #[default]
    Drop,
    /// Preserve the original bytes unchanged.
    Keep,
    /// Decode, transform, and encode the structure.
    Rewrite,
}

impl Retention {
    /// Returns whether the structure appears in the emitted artifact.
    #[must_use]
    pub const fn is_retained(self) -> bool {
        matches!(self, Self::Keep | Self::Rewrite)
    }

    /// Returns whether the structure must pass through a transformer.
    #[must_use]
    pub const fn requires_rewrite(self) -> bool {
        matches!(self, Self::Rewrite)
    }

    /// Returns whether original bytes are preserved unchanged.
    #[must_use]
    pub const fn keeps_verbatim(self) -> bool {
        matches!(self, Self::Keep)
    }
}

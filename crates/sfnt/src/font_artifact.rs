//! Immutable bytes exchanged between pipeline boundaries.

use std::{fmt, sync::Arc};

/// An immutable owned font byte sequence.
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct FontArtifact {
    bytes: Arc<[u8]>,
}

impl FontArtifact {
    /// Takes ownership of font bytes.
    #[must_use]
    pub fn new(bytes: impl Into<Arc<[u8]>>) -> Self {
        Self {
            bytes: bytes.into(),
        }
    }

    /// Returns the number of bytes in the artifact.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the artifact contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Borrows the complete immutable font byte sequence.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl AsRef<[u8]> for FontArtifact {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl fmt::Debug for FontArtifact {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FontArtifact")
            .field("length", &self.len())
            .finish_non_exhaustive()
    }
}

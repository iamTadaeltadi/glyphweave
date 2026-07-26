//! Indexed semantic GSUB Lookup collection.

use glyphweave_sfnt::{Decoded, LayoutCause, LayoutTableKind};

use crate::{GsubLookup, LookupList, layout_error};

/// Semantic GSUB Lookups in stable LookupList index order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GsubLookupList {
    lookups: Vec<GsubLookup>,
}

impl GsubLookupList {
    /// Decodes every Lookup while retaining its original zero-based index.
    /// # Errors
    /// For a non-GSUB list or any malformed semantic Lookup.
    pub fn decode(bytes: &[u8], list: &LookupList) -> Decoded<Self> {
        if list.table() != LayoutTableKind::Substitution {
            return Err(layout_error(
                0,
                LayoutCause::UnexpectedLayoutTableKind {
                    expected: LayoutTableKind::Substitution,
                    actual: list.table(),
                },
            ));
        }
        let lookups = list
            .lookups()
            .iter()
            .map(|lookup| GsubLookup::decode(bytes, lookup))
            .collect::<Decoded<Vec<_>>>()?;
        Ok(Self { lookups })
    }

    /// Returns a Lookup by its original zero-based LookupList index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&GsubLookup> {
        self.lookups.get(usize::from(index))
    }

    /// Returns semantic Lookups in original index order.
    #[must_use]
    pub fn lookups(&self) -> &[GsubLookup] {
        &self.lookups
    }

    /// Returns the number of indexed Lookups.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lookups.len()
    }

    /// Returns whether no Lookups are indexed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }
}

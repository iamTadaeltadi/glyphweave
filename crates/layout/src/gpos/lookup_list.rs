//! Indexed semantic GPOS Lookup collection.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutTarget, Reader};

use crate::{GposLookup, relative_target_span};

/// Semantic GPOS Lookups in stable `LookupList` index order.
///
/// ```
/// use glyphweave_layout::GposLookupList;
///
/// let empty: GposLookupList = GposLookupList::default();
/// assert!(empty.is_empty());
/// assert_eq!(empty.len(), 0);
/// assert!(empty.lookup(0).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GposLookupList<G = GlyphId> {
    lookups: Vec<GposLookup<G>>,
}

impl GposLookupList<GlyphId> {
    /// Decodes a GPOS `LookupList` at the beginning of `span`, retaining each Lookup's index.
    ///
    /// # Errors
    ///
    /// For truncation, an out-of-range Lookup offset, or any malformed nested Lookup.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let count = reader.u16()?;
        let mut offsets = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            let position = reader.offset();
            offsets.push((position, reader.u16()?));
        }
        let minimum = reader.offset() - span.start();
        let mut lookups = Vec::with_capacity(offsets.len());
        for (position, offset) in offsets {
            let target =
                relative_target_span(span, position, offset, minimum, LayoutTarget::Lookup)?;
            lookups.push(GposLookup::decode(bytes, target)?);
        }
        Ok(Self { lookups })
    }
}

impl<G> GposLookupList<G> {
    /// Returns a Lookup by its original zero-based `LookupList` index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&GposLookup<G>> {
        self.lookups.get(usize::from(index))
    }

    /// Returns semantic Lookups in original index order.
    #[must_use]
    pub fn lookups(&self) -> &[GposLookup<G>] {
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

#[cfg(test)]
mod tests;

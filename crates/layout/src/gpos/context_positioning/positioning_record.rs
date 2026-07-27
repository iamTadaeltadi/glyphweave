//! One ordered contextual GPOS action.

/// A contextual positioning action applying another Lookup to one input position.
///
/// This is the GPOS `SequenceLookupRecord`: a sequence index selecting an input
/// position and a `LookupList` index naming the nested Lookup applied there. The
/// referenced Lookup is retained only as data and is resolved later in the pipeline.
///
/// # Examples
/// ```
/// use glyphweave_layout::ContextPositioningRecord;
///
/// let record = ContextPositioningRecord::new(2, 5);
/// assert_eq!(record.sequence_index(), 2);
/// assert_eq!(record.lookup_index(), 5);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextPositioningRecord {
    sequence_index: u16,
    lookup_index: u16,
}

impl ContextPositioningRecord {
    /// Creates an action without changing either `LookupList` coordinate.
    #[must_use]
    pub const fn new(sequence_index: u16, lookup_index: u16) -> Self {
        Self {
            sequence_index,
            lookup_index,
        }
    }

    /// Returns the zero-based input position receiving the nested Lookup.
    #[must_use]
    pub const fn sequence_index(self) -> u16 {
        self.sequence_index
    }

    /// Returns the referenced zero-based `LookupList` index.
    #[must_use]
    pub const fn lookup_index(self) -> u16 {
        self.lookup_index
    }
}

#[cfg(test)]
mod tests {
    use super::ContextPositioningRecord;

    #[test]
    fn preserves_both_contextual_coordinates() {
        let record = ContextPositioningRecord::new(3, 17);
        assert_eq!(record.sequence_index(), 3);
        assert_eq!(record.lookup_index(), 17);
    }
}

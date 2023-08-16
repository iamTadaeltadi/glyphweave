//! One ordered contextual GSUB action.

/// A contextual action applying another Lookup to one input sequence position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubstitutionRecord {
    sequence_index: u16,
    lookup_index: u16,
}

impl SubstitutionRecord {
    /// Creates an action without changing either LookupList coordinate.
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

    /// Returns the referenced zero-based LookupList index.
    #[must_use]
    pub const fn lookup_index(self) -> u16 {
        self.lookup_index
    }
}

#[cfg(test)]
mod tests {
    use super::SubstitutionRecord;

    #[test]
    fn preserves_both_contextual_coordinates() {
        let record = SubstitutionRecord::new(3, 17);
        assert_eq!(record.sequence_index(), 3);
        assert_eq!(record.lookup_index(), 17);
    }
}

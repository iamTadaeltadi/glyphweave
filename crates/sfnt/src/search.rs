//! Derived binary-search parameters in an SFNT Table Directory header.

#![allow(clippy::redundant_pub_crate)]

/// The greatest table count whose byte ranges fit the header's `u16` fields.
pub(crate) const MAX_TABLES: u16 = 4_095;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SearchParameters {
    pub(crate) search_range: u16,
    pub(crate) entry_selector: u16,
    pub(crate) range_shift: u16,
}

impl SearchParameters {
    pub(crate) fn derive(table_count: u16) -> Option<Self> {
        if table_count == 0 {
            return Some(Self {
                search_range: 0,
                entry_selector: 0,
                range_shift: 0,
            });
        }
        if table_count > MAX_TABLES {
            return None;
        }

        let entry_selector = u16::BITS - 1 - table_count.leading_zeros();
        let power_of_two = 1_u32 << entry_selector;
        let search_range = power_of_two * 16;
        let range_shift = u32::from(table_count) * 16 - search_range;
        Some(Self {
            search_range: u16::try_from(search_range).ok()?,
            entry_selector: u16::try_from(entry_selector).ok()?,
            range_shift: u16::try_from(range_shift).ok()?,
        })
    }

    pub(crate) const fn into_array(self) -> [u16; 3] {
        [self.search_range, self.entry_selector, self.range_shift]
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_TABLES, SearchParameters};

    #[test]
    fn derives_zero_and_power_of_two_counts() {
        assert_eq!(
            SearchParameters::derive(0),
            Some(SearchParameters {
                search_range: 0,
                entry_selector: 0,
                range_shift: 0,
            })
        );
        assert_eq!(
            SearchParameters::derive(8),
            Some(SearchParameters {
                search_range: 128,
                entry_selector: 3,
                range_shift: 0,
            })
        );
    }

    #[test]
    fn derives_non_power_of_two_count() {
        assert_eq!(
            SearchParameters::derive(13),
            Some(SearchParameters {
                search_range: 128,
                entry_selector: 3,
                range_shift: 80,
            })
        );
    }

    #[test]
    fn rejects_unrepresentable_table_counts() {
        assert!(SearchParameters::derive(MAX_TABLES).is_some());
        assert_eq!(SearchParameters::derive(MAX_TABLES + 1), None);
    }
}

//! Deterministic totality properties for Lookup and LookupList decoding.

use glyphweave_layout::{Lookup, LookupList};
use glyphweave_sfnt::{ByteSpan, LayoutTableKind};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4c4f_4f4b),
        ..Config::default()
    }
}

fn table(positioning: bool) -> LayoutTableKind {
    if positioning {
        LayoutTableKind::Positioning
    } else {
        LayoutTableKind::Substitution
    }
}

fn is_effective_type(table: LayoutTableKind, lookup_type: u16) -> bool {
    match table {
        LayoutTableKind::Substitution => (1..=8).contains(&lookup_type) && lookup_type != 7,
        LayoutTableKind::Positioning => (1..=9).contains(&lookup_type) && lookup_type != 9,
        _ => false,
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_lookup_spans_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..257),
        start in 0..300usize,
        length in 0..300usize,
        positioning in any::<bool>(),
    ) {
        let span = ByteSpan::between(start, start + length).unwrap();
        let table = table(positioning);
        if let Ok(lookup) = Lookup::decode(&bytes, span, table) {
            prop_assert!(!lookup.is_empty());
            prop_assert!(is_effective_type(table, lookup.lookup_type()));
            let bounded = lookup.subtables().iter().all(|subtable| {
                subtable.start() >= span.start() && subtable.end() == span.end()
            });
            prop_assert!(bounded);
        }
    }

    #[test]
    fn arbitrary_lookup_list_spans_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..257),
        start in 0..300usize,
        length in 0..300usize,
        positioning in any::<bool>(),
    ) {
        let span = ByteSpan::between(start, start + length).unwrap();
        let table = table(positioning);
        if let Ok(list) = LookupList::decode(&bytes, span, table) {
            prop_assert_eq!(list.table(), table);
            let canonical = list.lookups().iter().all(|lookup| {
                !lookup.is_empty() && is_effective_type(table, lookup.lookup_type())
            });
            prop_assert!(canonical);
        }
    }
}

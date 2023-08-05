//! Deterministic totality properties for Lookup and LookupList decoding.

use glyphweave_layout::{Lookup, LookupList, MarkFilter};
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

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
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

    #[test]
    fn generated_flag_words_resolve_mark_precedence(
        right_to_left in any::<bool>(),
        ignore_base in any::<bool>(),
        ignore_ligatures in any::<bool>(),
        ignore_marks in any::<bool>(),
        filtering_set in proptest::option::of(any::<u16>()),
        attachment_class in any::<u8>(),
    ) {
        let low = u16::from(right_to_left)
            | u16::from(ignore_base) << 1
            | u16::from(ignore_ligatures) << 2
            | u16::from(ignore_marks) << 3
            | u16::from(filtering_set.is_some()) << 4;
        let raw_flags = low | u16::from(attachment_class) << 8;
        let offset = if filtering_set.is_some() { 10 } else { 8 };
        let mut bytes = Vec::new();
        push_u16(&mut bytes, 1);
        push_u16(&mut bytes, raw_flags);
        push_u16(&mut bytes, 1);
        push_u16(&mut bytes, offset);
        if let Some(set) = filtering_set {
            push_u16(&mut bytes, set);
        }
        bytes.push(0xfe);

        let span = ByteSpan::between(0, bytes.len()).unwrap();
        let lookup = Lookup::decode(&bytes, span, LayoutTableKind::Substitution).unwrap();
        let class = (attachment_class != 0).then_some(attachment_class);
        prop_assert_eq!(
            lookup.flags().mark_filter(),
            MarkFilter::resolve(ignore_marks, filtering_set, class)
        );
    }
}

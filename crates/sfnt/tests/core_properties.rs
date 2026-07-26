//! Deterministic property checks for foundational SFNT value types.

use glyphweave_sfnt::ByteSpan;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x5f4e_5401),
        ..Config::default()
    }
}

fn bounded_span() -> impl Strategy<Value = (Vec<u8>, ByteSpan)> {
    collection::vec(any::<u8>(), 0..257)
        .prop_flat_map(|bytes| {
            let positions = 0..=bytes.len();
            (Just(bytes), positions.clone(), positions)
        })
        .prop_map(|(bytes, first, second)| {
            let start = first.min(second);
            let end = first.max(second);
            (bytes, ByteSpan::between(start, end).unwrap())
        })
}

fn span_pair() -> impl Strategy<Value = (ByteSpan, ByteSpan)> {
    (0usize..4096, 0usize..4096, 0usize..4096, 0usize..4096).prop_map(|(a, b, c, d)| {
        let first = ByteSpan::between(a.min(b), a.max(b)).unwrap();
        let second = ByteSpan::between(c.min(d), c.max(d)).unwrap();
        (first, second)
    })
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn bounded_span_slicing_matches_native_ranges((bytes, span) in bounded_span()) {
        let observed = span.slice(&bytes).unwrap();
        prop_assert_eq!(observed, &bytes[span.start()..span.end()]);
        prop_assert_eq!(observed.len(), span.length());
        prop_assert_eq!(observed.is_empty(), span.is_empty());
    }


    #[test]
    fn span_join_and_overlap_match_interval_algebra((first, second) in span_pair()) {
        let joined = first.join(second);
        prop_assert_eq!(joined, second.join(first));
        prop_assert_eq!(joined.start(), first.start().min(second.start()));
        prop_assert_eq!(joined.end(), first.end().max(second.end()));

        let intersection_start = first.start().max(second.start());
        let intersection_end = first.end().min(second.end());
        let expected_overlap = intersection_start < intersection_end;
        prop_assert_eq!(first.overlaps(second), expected_overlap);
        prop_assert_eq!(first.overlaps(second), second.overlaps(first));
    }

    #[test]
    fn sized_span_construction_matches_checked_addition(
        start in any::<usize>(),
        length in any::<usize>(),
    ) {
        match (start.checked_add(length), ByteSpan::sized(start, length)) {
            (Some(end), Some(span)) => {
                prop_assert_eq!((span.start(), span.end()), (start, end));
                prop_assert_eq!(span.length(), length);
            }
            (None, None) => {}
            pair => prop_assert!(false, "construction mismatch: {pair:?}"),
        }
    }
}

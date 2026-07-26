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

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn bounded_span_slicing_matches_native_ranges((bytes, span) in bounded_span()) {
        let observed = span.slice(&bytes).unwrap();
        prop_assert_eq!(observed, &bytes[span.start()..span.end()]);
        prop_assert_eq!(observed.len(), span.length());
        prop_assert_eq!(observed.is_empty(), span.is_empty());
    }
}

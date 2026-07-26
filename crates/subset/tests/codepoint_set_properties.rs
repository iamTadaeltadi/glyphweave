//! Deterministic properties for requested Codepoint Sets.

use std::collections::BTreeSet;

use glyphweave_subset::CodepointSet;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4350_5345),
        ..Config::default()
    }
}

fn unicode_scalars() -> impl Strategy<Value = Vec<char>> {
    collection::vec(any::<char>(), 0..1025)
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn collected_codepoints_match_an_ordered_set_model(scalars in unicode_scalars()) {
        let codepoints: CodepointSet = scalars.iter().copied().collect();
        let expected: BTreeSet<_> = scalars.into_iter().collect();

        prop_assert_eq!(codepoints.len(), expected.len());
        prop_assert_eq!(codepoints.is_empty(), expected.is_empty());
        prop_assert_eq!(
            codepoints.iter().collect::<Vec<_>>(),
            expected.into_iter().collect::<Vec<_>>(),
        );
    }
}

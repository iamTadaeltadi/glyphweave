//! Deterministic totality properties for shared layout-list decoders.

use glyphweave_layout::{Feature, LanguageSystem, Script};
use glyphweave_sfnt::ByteSpan;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4c49_5354),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_language_system_spans_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..257),
        start in 0..300usize,
        length in 0..300usize,
    ) {
        let span = ByteSpan::between(start, start + length).unwrap();
        if let Ok(language) = LanguageSystem::decode(&bytes, span) {
            prop_assert!(language.feature_indices().is_sorted());
        }
    }

    #[test]
    fn arbitrary_feature_spans_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..257),
        start in 0..300usize,
        length in 0..300usize,
    ) {
        let span = ByteSpan::between(start, start + length).unwrap();
        if let Ok(feature) = Feature::decode(&bytes, span) {
            prop_assert!(feature.lookup_indices().is_sorted());
            prop_assert!(feature
                .parameters_position()
                .is_none_or(|position| span.contains(position)));
        }
    }

    #[test]
    fn arbitrary_script_spans_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..257),
        start in 0..300usize,
        length in 0..300usize,
    ) {
        let span = ByteSpan::between(start, start + length).unwrap();
        if let Ok(script) = Script::decode(&bytes, span) {
            prop_assert!(script
                .languages()
                .windows(2)
                .all(|pair| pair[0].0 < pair[1].0));
            if let Some(default) = script.default_language() {
                prop_assert!(default.feature_indices().is_sorted());
            }
            for (_, language) in script.languages() {
                prop_assert!(language.feature_indices().is_sorted());
            }
        }
    }
}

//! Deterministic properties for immutable Font Artifacts.

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use glyphweave_sfnt::FontArtifact;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 128,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4152_5446),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn artifacts_detach_from_sources_and_compare_by_bytes(
        mut source in collection::vec(any::<u8>(), 1..4097),
    ) {
        let expected = source.clone();
        let borrowed = FontArtifact::new(source.as_slice());
        let owned = FontArtifact::new(expected.clone());
        source.fill(0);

        prop_assert_eq!(borrowed.as_bytes(), expected.as_slice());
        prop_assert_eq!(&borrowed, &owned);

        let mut borrowed_hash = DefaultHasher::new();
        let mut owned_hash = DefaultHasher::new();
        borrowed.hash(&mut borrowed_hash);
        owned.hash(&mut owned_hash);
        prop_assert_eq!(borrowed_hash.finish(), owned_hash.finish());
        prop_assert_eq!(format!("{borrowed:?}"), format!("FontArtifact {{ length: {}, .. }}", expected.len()));
    }
}

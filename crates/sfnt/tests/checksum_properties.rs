//! Deterministic properties for OpenType checksums.

use glyphweave_sfnt::Checksum;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4348_4b53),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn explicit_word_padding_does_not_change_checksums(
        bytes in collection::vec(any::<u8>(), 0..4097),
    ) {
        let mut padded = bytes.clone();
        padded.resize((padded.len() + 3) & !3, 0);
        prop_assert_eq!(Checksum::compute(&bytes), Checksum::compute(&padded));
    }
}

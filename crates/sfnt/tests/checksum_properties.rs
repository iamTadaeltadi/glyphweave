//! Deterministic properties for OpenType checksums.

use glyphweave_sfnt::{Checksum, Tag};
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


    #[test]
    fn head_checksum_is_independent_of_adjustment_bytes(
        mut bytes in collection::vec(any::<u8>(), 12..4097),
        first_adjustment in any::<u32>(),
        second_adjustment in any::<u32>(),
    ) {
        bytes[8..12].copy_from_slice(&first_adjustment.to_be_bytes());
        let first = Checksum::compute_for(Tag::new(*b"head"), &bytes);
        bytes[8..12].copy_from_slice(&second_adjustment.to_be_bytes());
        let second = Checksum::compute_for(Tag::new(*b"head"), &bytes);
        prop_assert_eq!(first, second);
    }
}

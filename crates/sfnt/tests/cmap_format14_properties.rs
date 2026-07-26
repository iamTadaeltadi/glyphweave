//! Deterministic properties for total format-14 decoding.

use glyphweave_sfnt::CmapFormat14;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x434d_3134),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_bytes_and_offsets_are_decoded_totally(
        bytes in collection::vec(any::<u8>(), 0..513),
        offset in any::<usize>(),
    ) {
        if let Ok(decoded) = CmapFormat14::decode(&bytes, offset) {
            prop_assert_eq!(decoded.span().start(), offset);
            prop_assert!(decoded.span().end() <= bytes.len());
            prop_assert!(decoded.span().length() >= 10);
            let maximum_records = (decoded.span().length() - 10) / 11;
            let record_count = usize::try_from(decoded.selector_record_count()).unwrap();
            prop_assert!(record_count <= maximum_records);
            prop_assert!(decoded
                .variation_selector_map()
                .default_ranges()
                .all(|(_, range)| range.start() <= range.end()));
        }
    }
}

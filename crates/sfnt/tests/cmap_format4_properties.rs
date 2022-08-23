//! Deterministic properties for total format-4 decoding.

use glyphweave_sfnt::CmapFormat4;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x434d_4150),
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
        if let Ok(decoded) = CmapFormat4::decode(&bytes, offset) {
            prop_assert_eq!(decoded.span().start(), offset);
            prop_assert!(decoded.span().end() <= bytes.len());
            prop_assert_eq!(decoded.span().length() % 2, 0);
            prop_assert!(decoded.span().length() >= 24);
            prop_assert!(decoded.segment_count() > 0);
            prop_assert!(decoded
                .character_map()
                .iter()
                .all(|(_, glyph)| !glyph.is_notdef()));
        }
    }
}

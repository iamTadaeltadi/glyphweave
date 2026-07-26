//! Deterministic properties for total format-12 decoding.

use glyphweave_sfnt::CmapFormat12;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x434d_3132),
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
        if let Ok(decoded) = CmapFormat12::decode(&bytes, offset) {
            prop_assert_eq!(decoded.span().start(), offset);
            prop_assert!(decoded.span().end() <= bytes.len());
            prop_assert!(decoded.span().length() >= 16);
            let group_count = usize::try_from(decoded.group_count()).unwrap();
            prop_assert_eq!(decoded.span().length(), 16 + group_count * 12);
            prop_assert!(decoded
                .character_map()
                .iter()
                .all(|(_, glyph)| !glyph.is_notdef()));
        }
    }
}

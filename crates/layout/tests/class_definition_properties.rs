//! Deterministic properties for Class Definition canonicalization and round trips.

use glyphweave_layout::ClassDefinition;
use glyphweave_sfnt::GlyphId;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x434c_4153),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_bytes_and_offsets_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..513),
        offset in any::<usize>(),
    ) {
        if let Ok(classes) = ClassDefinition::decode(&bytes, offset) {
            prop_assert!(classes.assignments().all(|(_, class)| class != 0));
            prop_assert!(classes
                .assignments()
                .map(|(glyph, _)| glyph)
                .is_sorted());
        }
    }

    #[test]
    fn canonical_values_round_trip(
        assignments in collection::vec((any::<u16>(), any::<u16>()), 0..257),
    ) {
        let classes = ClassDefinition::from_assignments(
            assignments
                .into_iter()
                .map(|(glyph, class)| (GlyphId::new(glyph), class)),
        );
        let encoded = classes.encode();
        let decoded = ClassDefinition::decode(&encoded, 0).unwrap();
        prop_assert_eq!(decoded, classes);
    }
}

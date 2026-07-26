//! Deterministic properties for resolved Character Maps.

use std::collections::BTreeMap;

use glyphweave_sfnt::{CharacterMap, GlyphId};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x434d_4150),
        ..Config::default()
    }
}

fn mappings() -> impl Strategy<Value = Vec<(char, u16)>> {
    collection::vec((any::<char>(), any::<u16>()), 0..257)
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn character_maps_match_a_canonical_ordered_model(
        entries in mappings(),
        query in any::<char>(),
    ) {
        let mut expected = BTreeMap::new();
        let mut map = CharacterMap::new();
        for (codepoint, raw_glyph) in entries {
            let glyph = GlyphId::new(raw_glyph);
            map.insert(codepoint, glyph);
            if glyph.is_notdef() {
                expected.remove(&codepoint);
            } else {
                expected.insert(codepoint, glyph);
            }
        }

        let expected_query = expected.get(&query).copied().unwrap_or(GlyphId::NOTDEF);
        prop_assert_eq!(map.glyph_id(query), expected_query);
        prop_assert_eq!(map.len(), expected.len());
        prop_assert_eq!(map.iter().collect::<Vec<_>>(), expected.into_iter().collect::<Vec<_>>());
    }
}

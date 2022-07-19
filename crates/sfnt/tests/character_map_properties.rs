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

#[derive(Clone, Copy, Debug)]
enum MappingOperation {
    Insert(char, u16),
    Remove(char),
    RetainThrough(char),
}

fn mapping_operations() -> impl Strategy<Value = Vec<MappingOperation>> {
    collection::vec(
        prop_oneof![
            (any::<char>(), any::<u16>())
                .prop_map(|(codepoint, glyph)| MappingOperation::Insert(codepoint, glyph)),
            any::<char>().prop_map(MappingOperation::Remove),
            any::<char>().prop_map(MappingOperation::RetainThrough),
        ],
        0..129,
    )
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


    #[test]
    fn character_map_mutations_match_the_ordered_model(operations in mapping_operations()) {
        let mut expected = BTreeMap::new();
        let mut map = CharacterMap::new();
        for operation in operations {
            match operation {
                MappingOperation::Insert(codepoint, raw_glyph) => {
                    let glyph = GlyphId::new(raw_glyph);
                    map.insert(codepoint, glyph);
                    if glyph.is_notdef() {
                        expected.remove(&codepoint);
                    } else {
                        expected.insert(codepoint, glyph);
                    }
                }
                MappingOperation::Remove(codepoint) => {
                    prop_assert_eq!(map.remove(codepoint), expected.remove(&codepoint));
                }
                MappingOperation::RetainThrough(last) => {
                    map.retain(|codepoint, _| codepoint <= last);
                    expected.retain(|codepoint, _| *codepoint <= last);
                }
            }
            prop_assert_eq!(map.iter().collect::<Vec<_>>(), expected.clone().into_iter().collect::<Vec<_>>());
        }
    }
}

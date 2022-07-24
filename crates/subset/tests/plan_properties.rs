//! Deterministic properties for canonical Subset Plans.

use glyphweave_sfnt::{CharacterMap, FaceIndex, GlyphId, Tag};
use glyphweave_subset::{CodepointSet, Retention, SubsetPlan};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 192,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x504c_414e),
        ..Config::default()
    }
}

const fn retention(raw: u8) -> Retention {
    match raw % 3 {
        0 => Retention::Drop,
        1 => Retention::Keep,
        _ => Retention::Rewrite,
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn equivalent_models_produce_identical_valid_plans(
        face in any::<u32>(),
        requested in collection::btree_set(any::<char>(), 0..257),
        mappings in collection::btree_map(any::<char>(), any::<u16>(), 0..257),
        decisions in collection::btree_map(any::<u32>(), 0u8..3, 0..129),
        query_tag in any::<u32>(),
    ) {
        let codepoints: CodepointSet = requested.iter().copied().collect();
        let character_map: CharacterMap = mappings
            .iter()
            .map(|(&codepoint, &raw)| (codepoint, GlyphId::new(raw)))
            .collect();
        let table_decisions = decisions
            .iter()
            .map(|(&tag, &raw)| (Tag::from_u32(tag), retention(raw)))
            .collect::<Vec<_>>();

        let forward = SubsetPlan::from_character_map(
            FaceIndex::new(face),
            codepoints.clone(),
            &character_map,
            table_decisions.iter().copied(),
        );
        let reverse = SubsetPlan::from_character_map(
            FaceIndex::new(face),
            codepoints,
            &character_map,
            table_decisions.iter().rev().copied(),
        );

        prop_assert_eq!(&forward, &reverse);
        for codepoint in requested {
            prop_assert!(forward.glyphs().contains(character_map.glyph_id(codepoint)));
        }
        for glyph in forward.glyphs() {
            prop_assert!(forward.remap().new_glyph_id(glyph).is_some());
        }

        let expected_retention = decisions
            .get(&query_tag)
            .copied()
            .map(retention)
            .unwrap_or_default();
        prop_assert_eq!(forward.retention(Tag::from_u32(query_tag)), expected_retention);
        prop_assert!(forward.retentions().all(|(_, action)| action.is_retained()));
    }
}

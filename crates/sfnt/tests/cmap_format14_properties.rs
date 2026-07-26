//! Deterministic properties for total format-14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{CmapFormat14, GlyphId, VariationMapping, VariationSelector};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};
use support::{DefaultRange, NonDefaultMapping, encode_selector};

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

    #[test]
    fn generated_tables_match_their_variation_mapping_model(
        raw_selector in prop_oneof![0xfe00u32..=0xfe0f, 0xe0100u32..=0xe01ef],
        default_bases in collection::btree_set(0x4000u32..0x7f00, 0..9),
        non_defaults in collection::btree_map(0x8000u32..0xd800, any::<u16>(), 0..9),
        prefix in collection::vec(any::<u8>(), 0..33),
    ) {
        let defaults: Vec<_> = default_bases
            .iter()
            .copied()
            .map(|start| DefaultRange { start, additional: 0 })
            .collect();
        let mappings: Vec<_> = non_defaults
            .iter()
            .map(|(&base, &glyph)| NonDefaultMapping { base, glyph })
            .collect();
        let mut bytes = prefix;
        let offset = bytes.len();
        bytes.extend_from_slice(&encode_selector(raw_selector, &defaults, &mappings));

        let decoded = CmapFormat14::decode(&bytes, offset).unwrap();
        let selector = VariationSelector::new(char::from_u32(raw_selector).unwrap()).unwrap();
        let map = decoded.variation_selector_map();
        prop_assert_eq!(map.default_range_count(), default_bases.len());
        prop_assert_eq!(map.non_default_mapping_count(), non_defaults.len());
        for base in default_bases {
            let scalar = char::from_u32(base).unwrap();
            prop_assert_eq!(map.mapping(scalar, selector), Some(VariationMapping::Default));
        }
        for (base, raw_glyph) in non_defaults {
            let scalar = char::from_u32(base).unwrap();
            let glyph = GlyphId::new(raw_glyph);
            prop_assert_eq!(
                map.mapping(scalar, selector),
                Some(VariationMapping::NonDefault(glyph)),
            );
        }
        prop_assert_eq!(map.mapping('\u{3000}', selector), None);
    }
}

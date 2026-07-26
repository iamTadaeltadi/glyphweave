//! Deterministic properties for total format-12 decoding.

#[path = "support/cmap_format12.rs"]
pub mod support;

use glyphweave_sfnt::{CmapFormat12, GlyphId};
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

    #[test]
    fn generated_supplementary_groups_match_sequential_resolution(
        start in 0x1_0000u32..=0x10_ffbf,
        width in 0u32..64,
        start_glyph in 0u32..=65_472,
        language in any::<u32>(),
        prefix in collection::vec(any::<u8>(), 0..33),
    ) {
        let end = start + width;
        let mut bytes = prefix;
        let offset = bytes.len();
        bytes.extend_from_slice(&support::encode_groups(language, &[(start, end, start_glyph)]));

        let decoded = CmapFormat12::decode(&bytes, offset).unwrap();
        prop_assert_eq!(decoded.language(), language);
        prop_assert_eq!(decoded.group_count(), 1);
        let mut expected_length = 0;
        for codepoint in start..=end {
            let raw_glyph = start_glyph + (codepoint - start);
            let glyph = GlyphId::new(u16::try_from(raw_glyph).unwrap());
            let scalar = char::from_u32(codepoint).unwrap();
            prop_assert_eq!(decoded.character_map().glyph_id(scalar), glyph);
            if !glyph.is_notdef() {
                expected_length += 1;
            }
        }
        prop_assert_eq!(decoded.character_map().len(), expected_length);
    }
}

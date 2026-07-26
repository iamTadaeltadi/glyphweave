//! Deterministic properties for total format-4 decoding.

#[path = "support/cmap_format4.rs"]
pub mod support;

use glyphweave_sfnt::{CmapFormat4, GlyphId};
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

const fn direct_words(start: u16, end: u16, delta: u16, language: u16) -> [u16; 16] {
    [
        4, 32, language, 4, 4, 1, 0, // header
        end, 0xffff, // endCode
        0,      // reservedPad
        start, 0xffff, // startCode
        delta, 1, // idDelta
        0, 0, // idRangeOffset
    ]
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

    #[test]
    fn generated_direct_segments_match_modular_delta_resolution(
        start in 0u16..=0xd7bf,
        width in 0u16..64,
        delta in any::<u16>(),
        language in any::<u16>(),
        prefix in collection::vec(any::<u8>(), 0..33),
    ) {
        let end = start + width;
        let mut bytes = prefix;
        let offset = bytes.len();
        support::append_words(&mut bytes, &direct_words(start, end, delta, language));

        let decoded = CmapFormat4::decode(&bytes, offset).unwrap();
        prop_assert_eq!(decoded.language(), language);
        prop_assert_eq!(decoded.segment_count(), 2);
        let mut expected_length = 0;
        for codepoint in start..=end {
            let expected = GlyphId::new(codepoint.wrapping_add(delta));
            let scalar = char::from_u32(u32::from(codepoint)).unwrap();
            prop_assert_eq!(decoded.character_map().glyph_id(scalar), expected);
            if !expected.is_notdef() {
                expected_length += 1;
            }
        }
        prop_assert_eq!(decoded.character_map().len(), expected_length);
    }
}

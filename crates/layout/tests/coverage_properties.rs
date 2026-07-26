//! Deterministic properties for Coverage canonicalization and binary round trips.

use glyphweave_layout::Coverage;
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
        rng_seed: RngSeed::Fixed(0x434f_5645),
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
        if let Ok(coverage) = Coverage::decode(&bytes, offset) {
            prop_assert!(coverage.glyphs().windows(2).all(|pair| pair[0] < pair[1]));
        }
    }

    #[test]
    fn canonical_values_round_trip(
        raw_glyphs in collection::vec(any::<u16>(), 0..257),
    ) {
        let coverage = Coverage::from_glyphs(raw_glyphs.into_iter().map(GlyphId::new));
        let encoded = coverage.encode();
        let decoded = Coverage::decode(&encoded, 0).unwrap();
        prop_assert_eq!(decoded, coverage);
    }

    #[test]
    fn coverage_indices_are_exact_inverses(
        raw_glyphs in collection::vec(any::<u16>(), 0..257),
    ) {
        let coverage = Coverage::from_glyphs(raw_glyphs.into_iter().map(GlyphId::new));
        for (index, glyph) in coverage.glyphs().iter().copied().enumerate() {
            let index = u16::try_from(index).unwrap();
            prop_assert_eq!(coverage.coverage_index(glyph), Some(index));
            prop_assert_eq!(coverage.glyph_at(index), Some(glyph));
        }
    }

    #[test]
    fn encoder_always_chooses_the_shortest_legal_format(
        raw_glyphs in collection::vec(any::<u16>(), 0..257),
    ) {
        let coverage = Coverage::from_glyphs(raw_glyphs.into_iter().map(GlyphId::new));
        let runs = coverage.glyphs().iter().copied().fold(
            (0usize, None::<GlyphId>),
            |(runs, previous), glyph| {
                let continues = previous.and_then(|prior| prior.raw().checked_add(1))
                    == Some(glyph.raw());
                (runs + usize::from(!continues), Some(glyph))
            },
        ).0;
        let format_2_size = 4 + 6 * runs;
        let expected = u16::try_from(coverage.len())
            .map_or(format_2_size, |_| format_2_size.min(4 + 2 * coverage.len()));
        prop_assert_eq!(coverage.encode().len(), expected);
    }
}

//! Deterministic properties for total simple-glyph decoding.

use glyphweave_outline::SimpleGlyph;
use glyphweave_sfnt::ByteSpan;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x5347_4c59),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_bytes_and_spans_decode_totally(
        bytes in collection::vec(any::<u8>(), 0..513),
        start in any::<usize>(),
        length in 0usize..513,
    ) {
        let span = ByteSpan::sized(start, length)
            .unwrap_or_else(|| ByteSpan::empty_at(start));

        if let Ok(glyph) = SimpleGlyph::decode(&bytes, span) {
            prop_assert_eq!(glyph.span(), span);
            prop_assert!(glyph.span().end() <= bytes.len());
            prop_assert_eq!(glyph.contour_count(), glyph.contour_ends().len());
            prop_assert!(glyph.contour_ends().windows(2).all(|pair| pair[0] < pair[1]));
            prop_assert_eq!(
                glyph.contour_ends().last().copied().unwrap_or(0),
                u32::try_from(glyph.points().len()).unwrap(),
            );
            prop_assert!(glyph.instructions().len() <= glyph.span().length());
            for index in 0..glyph.contour_count() {
                prop_assert!(glyph.contour(index).is_some());
            }
            prop_assert!(glyph.contour(glyph.contour_count()).is_none());
        }
    }
}

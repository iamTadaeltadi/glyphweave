//! Deterministic properties for total composite-glyph decoding.

use glyphweave_outline::{ComponentPlacement, CompositeGlyph};
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
        rng_seed: RngSeed::Fixed(0x434f_4d50),
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

        if let Ok(glyph) = CompositeGlyph::decode(&bytes, span) {
            prop_assert_eq!(glyph.span(), span);
            prop_assert!(glyph.span().end() <= bytes.len());
            prop_assert!(!glyph.components().is_empty());
            prop_assert_eq!(glyph.component_count(), glyph.components().len());
            let first_is_offset = matches!(
                glyph.components()[0].placement(),
                ComponentPlacement::Offset { .. }
            );
            prop_assert!(first_is_offset);
            prop_assert!(glyph.instructions().len() <= glyph.span().length());
        }
    }
}

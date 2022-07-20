//! Deterministic properties for retained Glyph Sets.

use std::collections::BTreeSet;

use glyphweave_outline::GlyphSet;
use glyphweave_sfnt::GlyphId;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x474c_5953),
        ..Config::default()
    }
}

fn glyph_ids() -> impl Strategy<Value = Vec<u16>> {
    collection::vec(any::<u16>(), 0..1025)
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn collected_glyph_sets_are_unique_sorted_and_nonempty(raw_glyphs in glyph_ids()) {
        let glyphs: GlyphSet = raw_glyphs.iter().copied().map(GlyphId::new).collect();
        let mut expected: BTreeSet<_> = raw_glyphs.into_iter().collect();
        expected.insert(0);

        prop_assert!(!glyphs.is_empty());
        prop_assert!(glyphs.contains(GlyphId::NOTDEF));
        prop_assert_eq!(glyphs.len(), expected.len());
        prop_assert_eq!(
            glyphs.iter().map(GlyphId::raw).collect::<Vec<_>>(),
            expected.into_iter().collect::<Vec<_>>(),
        );
    }
}

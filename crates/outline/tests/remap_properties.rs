//! Deterministic properties for dense canonical Remaps.

use std::collections::BTreeSet;

use glyphweave_outline::{GlyphSet, Remap};
use glyphweave_sfnt::{GlyphId, NewGlyphId};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x524d_4150),
        ..Config::default()
    }
}

fn sparse_glyph_set() -> impl Strategy<Value = GlyphSet> {
    collection::vec(any::<u16>(), 0..2049)
        .prop_map(|raw_glyphs| raw_glyphs.into_iter().map(GlyphId::new).collect())
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn remaps_are_total_dense_injective_and_reversible(
        glyphs in sparse_glyph_set(),
        query in any::<u16>(),
    ) {
        let remap = Remap::new(&glyphs);
        let pairs = remap.iter().collect::<Vec<_>>();
        let emitted = pairs.iter().map(|pair| pair.1).collect::<BTreeSet<_>>();

        prop_assert_eq!(pairs.len(), glyphs.len());
        prop_assert_eq!(emitted.len(), glyphs.len());
        for (expected_raw, (old, new)) in pairs.into_iter().enumerate() {
            prop_assert_eq!(usize::from(new.raw()), expected_raw);
            prop_assert_eq!(remap.new_glyph_id(old), Some(new));
            prop_assert_eq!(remap.glyph_id(new), Some(old));
        }

        let queried = GlyphId::new(query);
        prop_assert_eq!(remap.new_glyph_id(queried).is_some(), glyphs.contains(queried));
        prop_assert_eq!(
            remap.glyph_id(NewGlyphId::new(u16::try_from(remap.len()).unwrap())),
            None,
        );
    }
}

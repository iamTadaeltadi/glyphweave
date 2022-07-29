//! Deterministic properties for typed font indices.

use glyphweave_sfnt::{GlyphId, NewGlyphId};
use proptest::{
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4944_5801),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn glyph_identity_types_preserve_the_full_unsigned_domain(
        first in any::<u16>(),
        second in any::<u16>(),
    ) {
        let old = GlyphId::new(first);
        let new = NewGlyphId::new(first);
        prop_assert_eq!(old.raw(), first);
        prop_assert_eq!(new.raw(), first);
        prop_assert_eq!(old.is_notdef(), first == 0);
        prop_assert_eq!(new.is_notdef(), first == 0);
        prop_assert_eq!(old.cmp(&GlyphId::new(second)), first.cmp(&second));
        prop_assert_eq!(new.cmp(&NewGlyphId::new(second)), first.cmp(&second));
    }
}

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

#[derive(Clone, Copy, Debug)]
enum GlyphOperation {
    Insert(u16),
    Remove(u16),
    RetainThrough(u16),
}

fn glyph_operations() -> impl Strategy<Value = Vec<GlyphOperation>> {
    collection::vec(
        prop_oneof![
            any::<u16>().prop_map(GlyphOperation::Insert),
            any::<u16>().prop_map(GlyphOperation::Remove),
            any::<u16>().prop_map(GlyphOperation::RetainThrough),
        ],
        0..257,
    )
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


    #[test]
    fn glyph_set_mutations_preserve_the_notdef_invariant(operations in glyph_operations()) {
        let mut glyphs = GlyphSet::new();
        let mut expected = BTreeSet::from([0u16]);
        for operation in operations {
            match operation {
                GlyphOperation::Insert(raw) => {
                    prop_assert_eq!(glyphs.insert(GlyphId::new(raw)), expected.insert(raw));
                }
                GlyphOperation::Remove(raw) => {
                    let expected_removed = raw != 0 && expected.remove(&raw);
                    prop_assert_eq!(glyphs.remove(GlyphId::new(raw)), expected_removed);
                }
                GlyphOperation::RetainThrough(last) => {
                    glyphs.retain(|glyph| glyph.raw() <= last);
                    expected.retain(|raw| *raw == 0 || *raw <= last);
                }
            }
            prop_assert!(glyphs.contains(GlyphId::NOTDEF));
            prop_assert_eq!(
                glyphs.iter().map(GlyphId::raw).collect::<Vec<_>>(),
                expected.iter().copied().collect::<Vec<_>>(),
            );
        }
    }
}

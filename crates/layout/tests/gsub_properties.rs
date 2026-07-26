//! Fixed-seed properties for GSUB closure, retention, and remapping.

use std::collections::{BTreeMap, BTreeSet};

use glyphweave_layout::{Coverage, GsubClosure, GsubLookupList, Lookup, LookupFlags, LookupList};
use glyphweave_sfnt::{ByteSpan, GlyphId, LayoutTableKind};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 256,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4753_5542),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn adding_initial_glyphs_never_shrinks_closure(
        mappings in collection::btree_map(any::<u16>(), any::<u16>(), 0..65),
        initial in collection::vec(any::<u16>(), 0..65),
        extra in collection::vec(any::<u16>(), 0..65),
    ) {
        let semantic = single_lookup(&mappings);
        let smaller = GsubClosure::compute(
            &semantic,
            initial.iter().copied().map(GlyphId::new),
            [0],
        ).unwrap();
        let larger = GsubClosure::compute(
            &semantic,
            initial.into_iter().chain(extra).map(GlyphId::new),
            [0],
        ).unwrap();

        prop_assert!(smaller.glyphs().all(|glyph| larger.contains(glyph)));
        prop_assert!(smaller.lookup_indices().all(|index| larger.contains_lookup(index)));
    }

    #[test]
    fn closure_equals_reference_graph_reachability(
        mappings in collection::btree_map(any::<u16>(), any::<u16>(), 0..65),
        initial in collection::vec(any::<u16>(), 0..65),
    ) {
        let semantic = single_lookup(&mappings);
        let closure = GsubClosure::compute(
            &semantic,
            initial.iter().copied().map(GlyphId::new),
            [0],
        ).unwrap();
        let expected = reference_closure(&mappings, initial);

        prop_assert_eq!(
            closure.glyphs().map(GlyphId::raw).collect::<Vec<_>>(),
            expected.into_iter().collect::<Vec<_>>(),
        );
    }
}

fn reference_closure(
    mappings: &BTreeMap<u16, u16>,
    initial: impl IntoIterator<Item = u16>,
) -> BTreeSet<u16> {
    let mut glyphs = initial.into_iter().collect::<BTreeSet<_>>();
    loop {
        let before = glyphs.len();
        let additions = mappings
            .iter()
            .filter(|(input, _)| glyphs.contains(input))
            .map(|(_, &output)| output)
            .collect::<Vec<_>>();
        glyphs.extend(additions);
        if glyphs.len() == before {
            return glyphs;
        }
    }
}

fn single_lookup(mappings: &BTreeMap<u16, u16>) -> GsubLookupList {
    let count = u16::try_from(mappings.len()).unwrap();
    let coverage_offset = 6 + 2 * count;
    let mut bytes = Vec::new();
    push(&mut bytes, 2);
    push(&mut bytes, coverage_offset);
    push(&mut bytes, count);
    for &output in mappings.values() {
        push(&mut bytes, output);
    }
    let coverage = Coverage::from_glyphs(mappings.keys().copied().map(GlyphId::new));
    bytes.extend(coverage.encode());
    let lookup = Lookup::new(
        LayoutTableKind::Substitution,
        1,
        LookupFlags::default(),
        [ByteSpan::between(0, bytes.len()).unwrap()],
    );
    let list = LookupList::new(LayoutTableKind::Substitution, [lookup]);
    GsubLookupList::decode(&bytes, &list).unwrap()
}

fn push(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend(value.to_be_bytes());
}

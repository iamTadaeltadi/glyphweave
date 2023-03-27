//! Fixed-seed properties for reachable composite cycles.

mod support;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::{Cause, GlyphId, OutlineCause, Stage};
use proptest::{
    prelude::*,
    test_runner::{Config, RngSeed},
};

use support::glyph_graph::{Node, decode_graph};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4359_434c),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_back_edges_report_the_active_ancestor(
        prefix_length in 0u16..64,
        cycle_length in 1u16..129,
    ) {
        let glyph_count = prefix_length + cycle_length;
        let mut nodes = Vec::with_capacity(usize::from(glyph_count));
        for parent in 0..glyph_count - 1 {
            nodes.push(Node::Composite(vec![parent + 1]));
        }
        nodes.push(Node::Composite(vec![prefix_length]));
        let glyf = decode_graph(&nodes);

        let diagnostic = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap_err();

        prop_assert_eq!(diagnostic.stage(), Stage::Closure);
        prop_assert_eq!(
            diagnostic.cause(),
            Cause::Outline(OutlineCause::CompositeGlyphCycle {
                parent: GlyphId::new(glyph_count - 1),
                component: GlyphId::new(prefix_length),
            })
        );
    }
}

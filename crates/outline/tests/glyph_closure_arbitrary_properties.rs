//! Fixed-seed totality properties for arbitrary component graphs.

mod support;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::{Cause, GlyphId, OutlineCause, Stage};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

use support::glyph_graph::{Node, decode_graph};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4752_4150),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn arbitrary_graphs_close_or_return_a_typed_failure(
        specifications in collection::vec(collection::vec(any::<u16>(), 0..5), 1..65),
        raw_roots in collection::vec(any::<u16>(), 0..17),
    ) {
        let glyph_count = u16::try_from(specifications.len()).unwrap();
        let nodes: Vec<_> = specifications
            .into_iter()
            .map(|children| {
                if children.is_empty() {
                    Node::Simple
                } else {
                    Node::Composite(children)
                }
            })
            .collect();
        let initial: GlyphSet = raw_roots
            .into_iter()
            .map(|root| GlyphId::new(root % glyph_count))
            .collect();
        let glyf = decode_graph(&nodes);

        match GlyphClosure::compute(&initial, &glyf) {
            Ok(closure) => {
                prop_assert!(initial.iter().all(|glyph| closure.contains(glyph)));
                prop_assert!(closure.iter().all(|glyph| glyph.raw() < glyph_count));
                for parent in closure.iter() {
                    if let Node::Composite(children) = &nodes[usize::from(parent.raw())] {
                        prop_assert!(children
                            .iter()
                            .all(|child| closure.contains(GlyphId::new(*child))));
                    }
                }
                prop_assert!(closure.maximum_component_depth() < glyph_count);
            }
            Err(diagnostic) => {
                prop_assert_eq!(diagnostic.stage(), Stage::Closure);
                prop_assert_eq!(diagnostic.span(), None);
                let is_graph_failure = matches!(
                    diagnostic.cause(),
                    Cause::Outline(OutlineCause::CompositeComponentOutOfBounds { .. })
                        | Cause::Outline(OutlineCause::CompositeGlyphCycle { .. })
                );
                prop_assert!(is_graph_failure);
            }
        }
    }
}

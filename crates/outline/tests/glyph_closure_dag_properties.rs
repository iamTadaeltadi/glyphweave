//! Fixed-seed properties for acyclic composite Glyph Closure.

mod support;

use std::collections::BTreeSet;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::GlyphId;
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
        rng_seed: RngSeed::Fixed(0x434c_4f53),
        ..Config::default()
    }
}

fn nodes_from(specifications: &[(bool, Vec<u16>)]) -> Vec<Node> {
    specifications
        .iter()
        .enumerate()
        .map(|(index, (empty, raw_children))| {
            if index == 0 || raw_children.is_empty() {
                if *empty { Node::Empty } else { Node::Simple }
            } else {
                let bound = u16::try_from(index).unwrap();
                let children: BTreeSet<_> =
                    raw_children.iter().map(|child| child % bound).collect();
                Node::Composite(children.into_iter().collect())
            }
        })
        .collect()
}

fn initial_set(raw_roots: &[u16], glyph_count: usize) -> GlyphSet {
    let bound = u16::try_from(glyph_count).unwrap();
    raw_roots
        .iter()
        .map(|root| GlyphId::new(root % bound))
        .collect()
}

fn model(nodes: &[Node], initial: &GlyphSet) -> (Vec<u16>, u16) {
    let mut retained: BTreeSet<_> = initial.iter().map(GlyphId::raw).collect();
    let mut pending: Vec<_> = retained.iter().copied().collect();
    while let Some(parent) = pending.pop() {
        if let Node::Composite(children) = &nodes[usize::from(parent)] {
            for &child in children {
                if retained.insert(child) {
                    pending.push(child);
                }
            }
        }
    }

    let mut depths = vec![0u16; nodes.len()];
    for (index, node) in nodes.iter().enumerate() {
        if let Node::Composite(children) = node {
            depths[index] = children
                .iter()
                .map(|child| depths[usize::from(*child)] + 1)
                .max()
                .unwrap_or(0);
        }
    }
    let maximum = initial
        .iter()
        .map(|root| depths[usize::from(root.raw())])
        .max()
        .unwrap_or(0);
    (retained.into_iter().collect(), maximum)
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn closure_matches_an_independent_dag_model(
        specifications in collection::vec(
            (any::<bool>(), collection::vec(any::<u16>(), 0..6)),
            1..65,
        ),
        raw_roots in collection::vec(any::<u16>(), 0..17),
    ) {
        let nodes = nodes_from(&specifications);
        let initial = initial_set(&raw_roots, nodes.len());
        let expected = model(&nodes, &initial);
        let glyf = decode_graph(&nodes);

        let closure = GlyphClosure::compute(&initial, &glyf).unwrap();
        let actual: Vec<_> = closure.iter().map(GlyphId::raw).collect();

        prop_assert_eq!(actual, expected.0);
        prop_assert_eq!(closure.maximum_component_depth(), expected.1);
    }
}

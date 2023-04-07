//! Integration coverage for cycle-safe composite Glyph Closure.

mod support;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::{Cause, GlyphId, OutlineCause, Stage};

use support::glyph_graph::{Node, decode_graph};

#[test]
fn notdef_leaf_forms_the_minimal_closure() {
    let glyf = decode_graph(&[Node::Simple]);

    let closure = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap();

    assert_eq!(closure.len(), 1);
    assert!(!closure.is_empty());
    assert!(closure.contains(GlyphId::NOTDEF));
    assert_eq!(closure.iter().collect::<Vec<_>>(), [GlyphId::NOTDEF]);
    assert_eq!(closure.glyphs(), &GlyphSet::new());
    assert_eq!(closure.maximum_component_depth(), 0);
}

#[test]
fn direct_components_join_the_initial_set() {
    let glyf = decode_graph(&[Node::Empty, Node::Simple, Node::Composite(vec![1])]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(2)).collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2]
    );
    assert_eq!(closure.maximum_component_depth(), 1);
}

#[test]
fn borrowed_closure_iteration_is_sorted_and_exact() {
    let glyf = decode_graph(&[Node::Empty, Node::Simple, Node::Composite(vec![1])]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(2)).collect();
    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    let mut glyphs = (&closure).into_iter();

    assert_eq!(glyphs.len(), 3);
    assert_eq!(glyphs.next(), Some(GlyphId::NOTDEF));
    assert_eq!(glyphs.next_back(), Some(GlyphId::new(2)));
    assert_eq!(glyphs.next(), Some(GlyphId::new(1)));
    assert_eq!(glyphs.next(), None);
}

#[test]
fn closure_follows_transitive_component_edges() {
    let glyf = decode_graph(&[
        Node::Empty,
        Node::Simple,
        Node::Composite(vec![1]),
        Node::Composite(vec![2]),
    ]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(3)).collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
    assert_eq!(closure.maximum_component_depth(), 2);
}

#[test]
fn shared_subgraphs_are_memoized_without_losing_depth() {
    let glyf = decode_graph(&[
        Node::Empty,
        Node::Simple,
        Node::Composite(vec![1]),
        Node::Composite(vec![1]),
        Node::Composite(vec![2, 3]),
    ]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(4)).collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
    assert_eq!(closure.maximum_component_depth(), 2);
}

#[test]
fn maximum_depth_follows_the_longest_branch() {
    let glyf = decode_graph(&[
        Node::Empty,
        Node::Simple,
        Node::Composite(vec![1]),
        Node::Composite(vec![2]),
        Node::Simple,
        Node::Composite(vec![4, 3]),
    ]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(5)).collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(closure.maximum_component_depth(), 3);
    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 3, 4, 5]
    );
}

#[test]
fn rejects_an_initial_root_outside_glyf() {
    let glyf = decode_graph(&[Node::Simple]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(1)).collect();

    let diagnostic = GlyphClosure::compute(&initial, &glyf).unwrap_err();

    assert_eq!(diagnostic.stage(), Stage::Closure);
    assert_eq!(diagnostic.span(), None);
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::GlyphClosureRootOutOfBounds {
            glyph: GlyphId::new(1),
            glyph_count: 1,
        })
    );
}

#[test]
fn rejects_glyf_without_mandatory_notdef() {
    let glyf = decode_graph(&[]);

    let diagnostic = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::GlyphClosureRootOutOfBounds {
            glyph: GlyphId::NOTDEF,
            glyph_count: 0,
        })
    );
}

#[test]
fn rejects_a_reachable_dangling_component() {
    let glyf = decode_graph(&[Node::Empty, Node::Composite(vec![2])]);
    let initial: GlyphSet = std::iter::once(GlyphId::new(1)).collect();

    let diagnostic = GlyphClosure::compute(&initial, &glyf).unwrap_err();

    assert_eq!(diagnostic.stage(), Stage::Closure);
    assert_eq!(diagnostic.span(), None);
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::CompositeComponentOutOfBounds {
            parent: GlyphId::new(1),
            component: GlyphId::new(2),
            glyph_count: 2,
        })
    );
}

#[test]
fn ignores_dangling_edges_outside_the_reachable_graph() {
    let glyf = decode_graph(&[Node::Simple, Node::Composite(vec![2])]);

    let closure = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap();

    assert_eq!(closure.iter().collect::<Vec<_>>(), [GlyphId::NOTDEF]);
    assert_eq!(closure.maximum_component_depth(), 0);
}

#[test]
fn rejects_a_self_referential_composite() {
    let glyf = decode_graph(&[Node::Composite(vec![0])]);

    let diagnostic = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap_err();

    assert_eq!(diagnostic.stage(), Stage::Closure);
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::CompositeGlyphCycle {
            parent: GlyphId::NOTDEF,
            component: GlyphId::NOTDEF,
        })
    );
}

#[test]
fn rejects_a_multi_glyph_cycle_at_its_back_edge() {
    let glyf = decode_graph(&[
        Node::Composite(vec![1]),
        Node::Composite(vec![2]),
        Node::Composite(vec![0]),
    ]);

    let diagnostic = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::CompositeGlyphCycle {
            parent: GlyphId::new(2),
            component: GlyphId::NOTDEF,
        })
    );
}

#[test]
fn ignores_cycles_outside_the_reachable_graph() {
    let glyf = decode_graph(&[
        Node::Simple,
        Node::Composite(vec![2]),
        Node::Composite(vec![1]),
    ]);

    let closure = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap();

    assert_eq!(closure.iter().collect::<Vec<_>>(), [GlyphId::NOTDEF]);
    assert_eq!(closure.maximum_component_depth(), 0);
}

#[test]
fn reports_dangling_components_in_source_order() {
    let glyf = decode_graph(&[Node::Composite(vec![2, 3]), Node::Simple]);

    let diagnostic = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::CompositeComponentOutOfBounds {
            parent: GlyphId::NOTDEF,
            component: GlyphId::new(2),
            glyph_count: 2,
        })
    );
}

#[test]
fn visits_initial_roots_in_glyph_id_order() {
    let glyf = decode_graph(&[
        Node::Simple,
        Node::Empty,
        Node::Composite(vec![4]),
        Node::Composite(vec![5]),
    ]);
    let initial: GlyphSet = [GlyphId::new(3), GlyphId::new(2)].into_iter().collect();

    let diagnostic = GlyphClosure::compute(&initial, &glyf).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::CompositeComponentOutOfBounds {
            parent: GlyphId::new(2),
            component: GlyphId::new(4),
            glyph_count: 4,
        })
    );
}

#[test]
fn later_roots_reuse_depths_computed_for_earlier_roots() {
    let glyf = decode_graph(&[
        Node::Empty,
        Node::Simple,
        Node::Composite(vec![1]),
        Node::Composite(vec![2]),
    ]);
    let initial: GlyphSet = [GlyphId::new(2), GlyphId::new(3)].into_iter().collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(closure.maximum_component_depth(), 2);
    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
}

#[test]
fn roots_completed_as_components_reuse_their_depth() {
    let glyf = decode_graph(&[
        Node::Empty,
        Node::Simple,
        Node::Composite(vec![3]),
        Node::Composite(vec![1]),
    ]);
    let initial: GlyphSet = [GlyphId::new(2), GlyphId::new(3)].into_iter().collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(closure.maximum_component_depth(), 2);
    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
}

#[test]
fn traverses_the_maximal_glyph_chain_without_recursion() {
    let mut nodes: Vec<_> = (1..u16::MAX)
        .map(|child| Node::Composite(vec![child]))
        .collect();
    nodes.push(Node::Simple);
    let glyf = decode_graph(&nodes);

    let closure = GlyphClosure::compute(&GlyphSet::new(), &glyf).unwrap();

    assert_eq!(closure.len(), usize::from(u16::MAX));
    assert_eq!(closure.maximum_component_depth(), u16::MAX - 1);
    assert!(closure.contains(GlyphId::new(u16::MAX - 1)));
}

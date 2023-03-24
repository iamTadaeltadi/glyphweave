//! Integration coverage for cycle-safe composite Glyph Closure.

use glyphweave_outline::{Glyf, GlyphClosure, GlyphSet, Loca};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LocaFormat, OutlineCause, Stage};

#[derive(Clone, Debug)]
enum Node {
    Empty,
    Simple,
    Composite(Vec<u16>),
}

fn decode_graph(nodes: &[Node]) -> Glyf {
    let mut glyf_bytes = Vec::new();
    let mut offsets = vec![0u32];
    for node in nodes {
        match node {
            Node::Empty => {}
            Node::Simple => glyf_bytes.extend([0; 10]),
            Node::Composite(children) => {
                glyf_bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
                for (index, child) in children.iter().enumerate() {
                    let flags = if index + 1 == children.len() {
                        0x0002u16
                    } else {
                        0x0022u16
                    };
                    glyf_bytes.extend(flags.to_be_bytes());
                    glyf_bytes.extend(child.to_be_bytes());
                    glyf_bytes.extend([0, 0]);
                }
            }
        }
        offsets.push(u32::try_from(glyf_bytes.len()).unwrap());
    }

    let mut bytes = Vec::with_capacity(offsets.len() * 4 + glyf_bytes.len());
    for offset in &offsets {
        bytes.extend(offset.to_be_bytes());
    }
    let loca_length = bytes.len();
    bytes.extend(glyf_bytes);
    let loca = Loca::decode(
        &bytes,
        ByteSpan::sized(0, loca_length).unwrap(),
        ByteSpan::sized(loca_length, bytes.len() - loca_length).unwrap(),
        u16::try_from(nodes.len()).unwrap(),
        LocaFormat::Long,
    )
    .unwrap();
    Glyf::decode(&bytes, &loca).unwrap()
}

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
    let initial: GlyphSet = [GlyphId::new(2)].into_iter().collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2]
    );
    assert_eq!(closure.maximum_component_depth(), 1);
}

#[test]
fn closure_follows_transitive_component_edges() {
    let glyf = decode_graph(&[
        Node::Empty,
        Node::Simple,
        Node::Composite(vec![1]),
        Node::Composite(vec![2]),
    ]);
    let initial: GlyphSet = [GlyphId::new(3)].into_iter().collect();

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
    let initial: GlyphSet = [GlyphId::new(4)].into_iter().collect();

    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 3, 4]
    );
    assert_eq!(closure.maximum_component_depth(), 2);
}

#[test]
fn rejects_an_initial_root_outside_glyf() {
    let glyf = decode_graph(&[Node::Simple]);
    let initial: GlyphSet = [GlyphId::new(1)].into_iter().collect();

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
fn rejects_a_reachable_dangling_component() {
    let glyf = decode_graph(&[Node::Empty, Node::Composite(vec![2])]);
    let initial: GlyphSet = [GlyphId::new(1)].into_iter().collect();

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

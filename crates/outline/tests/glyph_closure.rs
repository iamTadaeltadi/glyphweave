//! Integration coverage for cycle-safe composite Glyph Closure.

use glyphweave_outline::{Glyf, GlyphClosure, GlyphSet, Loca};
use glyphweave_sfnt::{ByteSpan, GlyphId, LocaFormat};

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

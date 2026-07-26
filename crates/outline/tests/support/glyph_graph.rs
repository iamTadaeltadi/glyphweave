use glyphweave_outline::{Glyf, Loca};
use glyphweave_sfnt::{ByteSpan, LocaFormat};

#[derive(Clone, Debug)]
#[allow(
    dead_code,
    reason = "each integration crate uses a different node subset"
)]
pub(crate) enum Node {
    Empty,
    Simple,
    Composite(Vec<u16>),
}

pub(crate) fn decode_graph(nodes: &[Node]) -> Glyf {
    let mut glyf_bytes = Vec::new();
    let mut offsets = vec![0u32];
    for node in nodes {
        encode_node(node, &mut glyf_bytes);
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

fn encode_node(node: &Node, bytes: &mut Vec<u8>) {
    match node {
        Node::Empty => {}
        Node::Simple => bytes.extend([0; 10]),
        Node::Composite(children) => {
            bytes.extend([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
            for (index, child) in children.iter().enumerate() {
                let flags = if index + 1 == children.len() {
                    0x0002u16
                } else {
                    0x0022u16
                };
                bytes.extend(flags.to_be_bytes());
                bytes.extend(child.to_be_bytes());
                bytes.extend([0, 0]);
            }
        }
    }
}

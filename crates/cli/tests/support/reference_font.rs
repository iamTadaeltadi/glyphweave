use glyphweave_outline::{Glyf, Loca};
use glyphweave_sfnt::{ByteSpan, Face, FaceIndex, LocaFormat, Tag};

pub(crate) const BYTES: &[u8] = include_bytes!("../fixtures/reference.ttf");

pub(crate) fn glyf() -> Glyf {
    let face = Face::select(BYTES, FaceIndex::FIRST).expect("reference face must decode");
    let head = table_span(&face, *b"head");
    let maxp = table_span(&face, *b"maxp");
    let loca_span = table_span(&face, *b"loca");
    let glyf_span = table_span(&face, *b"glyf");
    let format = LocaFormat::new(read_i16(head, 50)).expect("fixture loca format must be standard");
    let glyph_count = read_u16(maxp, 4);
    let loca = Loca::decode(BYTES, loca_span, glyf_span, glyph_count, format)
        .expect("reference loca must decode");
    Glyf::decode(BYTES, &loca).expect("reference glyf must decode")
}

pub(crate) fn table_span(face: &Face, tag: [u8; 4]) -> ByteSpan {
    face.record(Tag::new(tag))
        .unwrap_or_else(|| panic!("reference font must contain {}", Tag::new(tag)))
        .table_span()
}

fn read_i16(span: ByteSpan, offset: usize) -> i16 {
    i16::from_be_bytes(read_pair(span, offset))
}

fn read_u16(span: ByteSpan, offset: usize) -> u16 {
    u16::from_be_bytes(read_pair(span, offset))
}

fn read_pair(span: ByteSpan, offset: usize) -> [u8; 2] {
    let start = span.start() + offset;
    BYTES[start..start + 2]
        .try_into()
        .expect("fixture field must be in bounds")
}

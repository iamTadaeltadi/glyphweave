use glyphweave_sfnt::{ByteSpan, CmapFormat4, CmapFormat12, CmapFormat14, Face, FaceIndex};

use super::reference_font::{BYTES, table_span};

pub(crate) fn format4() -> CmapFormat4 {
    format4_from(BYTES)
}

pub(crate) fn format4_from(bytes: &[u8]) -> CmapFormat4 {
    CmapFormat4::decode(bytes, subtable_offset(bytes, 0, 3)).expect("reference cmap 4 must decode")
}

pub(crate) fn format12() -> CmapFormat12 {
    format12_from(BYTES)
}

pub(crate) fn format12_from(bytes: &[u8]) -> CmapFormat12 {
    CmapFormat12::decode(bytes, subtable_offset(bytes, 3, 10))
        .expect("reference cmap 12 must decode")
}

pub(crate) fn format14() -> CmapFormat14 {
    format14_from(BYTES)
}

pub(crate) fn format14_from(bytes: &[u8]) -> CmapFormat14 {
    CmapFormat14::decode(bytes, subtable_offset(bytes, 0, 5))
        .expect("reference cmap 14 must decode")
}

fn subtable_offset(bytes: &[u8], platform: u16, encoding: u16) -> usize {
    let face = Face::select(bytes, FaceIndex::FIRST).expect("reference face must decode");
    let cmap = table_span(&face, *b"cmap");
    let record_count = usize::from(read_u16(bytes, cmap, 2));
    for index in 0..record_count {
        let record = 4 + index * 8;
        if read_u16(bytes, cmap, record) == platform
            && read_u16(bytes, cmap, record + 2) == encoding
        {
            let relative = usize::try_from(read_u32(bytes, cmap, record + 4)).unwrap();
            return cmap.start() + relative;
        }
    }
    panic!("reference cmap must contain platform {platform} encoding {encoding}");
}

fn read_u16(bytes: &[u8], span: ByteSpan, offset: usize) -> u16 {
    u16::from_be_bytes(read_array(bytes, span, offset))
}

fn read_u32(bytes: &[u8], span: ByteSpan, offset: usize) -> u32 {
    u32::from_be_bytes(read_array(bytes, span, offset))
}

fn read_array<const N: usize>(bytes: &[u8], span: ByteSpan, offset: usize) -> [u8; N] {
    let start = span.start() + offset;
    bytes[start..start + N]
        .try_into()
        .expect("fixture field must be in bounds")
}

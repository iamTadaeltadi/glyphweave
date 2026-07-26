//! Representational limits for composite-glyph decoding.

use glyphweave_outline::CompositeGlyph;
use glyphweave_sfnt::ByteSpan;

#[test]
fn decodes_the_maximal_unsigned_component_count() {
    let component_count = usize::from(u16::MAX);
    let mut bytes = vec![0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0];
    for index in 0..component_count {
        let flags = if index + 1 < component_count {
            0x0022_u16
        } else {
            0x0002
        };
        bytes.extend(flags.to_be_bytes());
        bytes.extend(u16::try_from(index).unwrap().to_be_bytes());
        bytes.extend([0, 0]);
    }
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.component_count(), component_count);
    assert_eq!(
        glyph.components().last().unwrap().glyph_id().raw(),
        u16::MAX - 1
    );
}

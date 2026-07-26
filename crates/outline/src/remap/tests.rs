use super::Remap;
use crate::GlyphSet;
use glyphweave_sfnt::{GlyphId, NewGlyphId};

#[test]
fn minimal_remap_pins_notdef_to_zero() {
    let remap = Remap::new(&GlyphSet::new());
    assert_eq!(remap.len(), 1);
    assert!(!remap.is_empty());
    assert_eq!(
        remap.new_glyph_id(GlyphId::NOTDEF),
        Some(NewGlyphId::NOTDEF)
    );
    assert_eq!(remap.glyph_id(NewGlyphId::NOTDEF), Some(GlyphId::NOTDEF));
}

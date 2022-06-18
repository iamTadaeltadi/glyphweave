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

#[test]
fn sparse_inputs_receive_dense_sorted_outputs() {
    let glyphs: GlyphSet = [400, 7, 19].into_iter().map(GlyphId::new).collect();
    let remap = Remap::new(&glyphs);
    assert_eq!(remap.len(), 4);
    assert_eq!(
        remap.new_glyph_id(GlyphId::new(7)),
        Some(NewGlyphId::new(1))
    );
    assert_eq!(
        remap.new_glyph_id(GlyphId::new(19)),
        Some(NewGlyphId::new(2))
    );
    assert_eq!(
        remap.new_glyph_id(GlyphId::new(400)),
        Some(NewGlyphId::new(3))
    );
}

#[test]
fn discarded_and_out_of_range_ids_are_undefined() {
    let glyphs: GlyphSet = [7, 400].into_iter().map(GlyphId::new).collect();
    let remap = Remap::new(&glyphs);
    assert_eq!(remap.new_glyph_id(GlyphId::new(1)), None);
    assert_eq!(remap.new_glyph_id(GlyphId::new(401)), None);
    assert_eq!(remap.glyph_id(NewGlyphId::new(3)), None);
    assert_eq!(remap.glyph_id(NewGlyphId::new(u16::MAX)), None);
}

#[test]
fn iteration_exposes_both_mapping_directions() {
    let glyphs: GlyphSet = [30, 4].into_iter().map(GlyphId::new).collect();
    let remap = Remap::new(&glyphs);
    let pairs: Vec<_> = remap
        .iter()
        .map(|(old, new)| (old.raw(), new.raw()))
        .collect();
    assert_eq!(pairs, [(0, 0), (4, 1), (30, 2)]);
    assert_eq!(
        remap.iter().next_back(),
        Some((GlyphId::new(30), NewGlyphId::new(2)))
    );
}

#[test]
fn complete_u16_universe_remains_representable() {
    let glyphs: GlyphSet = (0..=u16::MAX).map(GlyphId::new).collect();
    let remap = Remap::new(&glyphs);
    assert_eq!(remap.len(), usize::from(u16::MAX) + 1);
    assert_eq!(
        remap.new_glyph_id(GlyphId::new(u16::MAX)),
        Some(NewGlyphId::new(u16::MAX))
    );
    assert_eq!(
        remap.glyph_id(NewGlyphId::new(u16::MAX)),
        Some(GlyphId::new(u16::MAX))
    );
    assert!(remap.iter().all(|(old, new)| old.raw() == new.raw()));
}

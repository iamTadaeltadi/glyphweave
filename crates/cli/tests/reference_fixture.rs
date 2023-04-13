//! Structural checks for the deterministic differential font.

mod support;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::{Face, FaceIndex, GlyphId};

use support::reference_font::{BYTES, glyf};

#[test]
fn reference_font_has_the_reviewed_outline_topology() {
    let face = Face::select(BYTES, FaceIndex::FIRST).unwrap();
    let glyf = glyf();

    assert_eq!(BYTES.len(), 1_316);
    assert_eq!(face.directory().len(), 10);
    assert_eq!(glyf.glyph_count(), 6);
    assert!(glyf.outline(GlyphId::new(1)).unwrap().is_empty());
    for raw in [0, 2, 3] {
        assert!(
            glyf.outline(GlyphId::new(raw))
                .unwrap()
                .as_simple()
                .is_some()
        );
    }
    let direct = glyf
        .outline(GlyphId::new(4))
        .unwrap()
        .as_composite()
        .unwrap();
    assert_eq!(
        direct
            .components()
            .iter()
            .map(|component| component.glyph_id().raw())
            .collect::<Vec<_>>(),
        [2, 3]
    );
    let nested = glyf
        .outline(GlyphId::new(5))
        .unwrap()
        .as_composite()
        .unwrap();
    assert_eq!(
        nested
            .components()
            .iter()
            .map(|component| component.glyph_id().raw())
            .collect::<Vec<_>>(),
        [4, 2]
    );
}

#[test]
fn direct_reference_composite_closes_over_both_children() {
    let initial: GlyphSet = std::iter::once(GlyphId::new(4)).collect();

    let closure = GlyphClosure::compute(&initial, &glyf()).unwrap();

    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 2, 3, 4]
    );
    assert_eq!(closure.maximum_component_depth(), 1);
}

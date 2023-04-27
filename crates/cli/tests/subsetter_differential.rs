//! Live composite-closure comparisons with reference subsetters.

mod support;

use glyphweave_outline::{GlyphClosure, GlyphSet};
use glyphweave_sfnt::GlyphId;

use support::{reference_font::glyf_from, reference_tool::subset};

#[test]
fn fonttools_retains_the_glyphweave_component_closure() {
    let Some(bytes) = subset("pyftsubset", 5) else {
        return;
    };

    assert_reference_closure(&bytes);
}

#[test]
fn harfbuzz_retains_the_glyphweave_component_closure() {
    let Some(bytes) = subset("hb-subset", 5) else {
        return;
    };

    assert_reference_closure(&bytes);
}

fn assert_reference_closure(bytes: &[u8]) {
    let glyf = glyf_from(bytes);
    let initial: GlyphSet = std::iter::once(GlyphId::new(5)).collect();
    let closure = GlyphClosure::compute(&initial, &glyf).unwrap();

    assert_eq!(glyf.glyph_count(), 6);
    for raw in [0, 1] {
        assert!(glyf.outline(GlyphId::new(raw)).unwrap().is_empty());
    }
    for raw in [2, 3] {
        assert!(
            glyf.outline(GlyphId::new(raw))
                .unwrap()
                .as_simple()
                .is_some()
        );
    }
    for raw in [4, 5] {
        assert!(
            glyf.outline(GlyphId::new(raw))
                .unwrap()
                .as_composite()
                .is_some()
        );
    }
    assert_eq!(component_ids(&glyf, 4), [2, 3]);
    assert_eq!(component_ids(&glyf, 5), [4, 2]);
    assert_eq!(
        closure.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 2, 3, 4, 5]
    );
    assert_eq!(closure.maximum_component_depth(), 2);
}

fn component_ids(glyf: &glyphweave_outline::Glyf, raw: u16) -> Vec<u16> {
    glyf.outline(GlyphId::new(raw))
        .unwrap()
        .as_composite()
        .unwrap()
        .components()
        .iter()
        .map(|component| component.glyph_id().raw())
        .collect()
}

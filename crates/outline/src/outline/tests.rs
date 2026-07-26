use glyphweave_sfnt::{ByteSpan, GlyphId};

use super::Outline;
use crate::{
    Component, ComponentOffsetMode, ComponentPlacement, ComponentTransform, CompositeGlyph,
    GlyphBounds, SimpleGlyph,
};

fn bounds() -> GlyphBounds {
    GlyphBounds::new(-2, -3, 5, 7).unwrap()
}

fn simple() -> SimpleGlyph {
    SimpleGlyph {
        span: ByteSpan::sized(4, 10).unwrap(),
        bounds: bounds(),
        contour_ends: Vec::new(),
        points: Vec::new(),
        instructions: Vec::new(),
        overlapping_contours: false,
    }
}

fn composite() -> CompositeGlyph {
    CompositeGlyph {
        span: ByteSpan::sized(20, 16).unwrap(),
        bounds: bounds(),
        components: vec![Component::new(
            GlyphId::new(3),
            ComponentPlacement::Offset {
                x: 0,
                y: 0,
                round_to_grid: false,
                mode: ComponentOffsetMode::Default,
            },
            ComponentTransform::identity(),
            false,
        )],
        instructions: Vec::new(),
        overlapping_components: false,
    }
}

#[test]
fn empty_outline_has_only_source_identity() {
    let span = ByteSpan::empty_at(12);
    let outline = Outline::Empty { span };

    assert!(outline.is_empty());
    assert_eq!(outline.span(), span);
    assert_eq!(outline.bounds(), None);
    assert_eq!(outline.as_simple(), None);
    assert_eq!(outline.as_composite(), None);
}

#[test]
fn simple_outline_preserves_decoded_value() {
    let glyph = simple();
    let outline = Outline::Simple(glyph.clone());

    assert!(!outline.is_empty());
    assert_eq!(outline.span(), glyph.span());
    assert_eq!(outline.bounds(), Some(bounds()));
    assert_eq!(outline.as_simple(), Some(&glyph));
    assert_eq!(outline.as_composite(), None);
}

#[test]
fn composite_outline_preserves_decoded_value() {
    let glyph = composite();
    let outline = Outline::Composite(glyph.clone());

    assert!(!outline.is_empty());
    assert_eq!(outline.span(), glyph.span());
    assert_eq!(outline.bounds(), Some(bounds()));
    assert_eq!(outline.as_simple(), None);
    assert_eq!(outline.as_composite(), Some(&glyph));
}

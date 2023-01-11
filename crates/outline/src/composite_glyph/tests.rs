use glyphweave_sfnt::{ByteSpan, GlyphId};

use super::CompositeGlyph;
use crate::{Component, ComponentOffsetMode, ComponentPlacement, ComponentTransform, GlyphBounds};

#[test]
fn accessors_preserve_unresolved_composite_identity() {
    let span = ByteSpan::between(12, 32).unwrap();
    let bounds = GlyphBounds::new(-3, -5, 8, 13).unwrap();
    let component = Component::new(
        GlyphId::new(17),
        ComponentPlacement::Offset {
            x: -3,
            y: 5,
            round_to_grid: false,
            mode: ComponentOffsetMode::Default,
        },
        ComponentTransform::identity(),
        true,
    );
    let glyph = CompositeGlyph {
        span,
        bounds,
        components: vec![component],
        instructions: vec![0x2a, 0x2b],
        overlapping_components: true,
    };

    assert_eq!(glyph.span(), span);
    assert_eq!(glyph.bounds(), bounds);
    assert_eq!(glyph.components(), [component]);
    assert_eq!(glyph.component_count(), 1);
    assert_eq!(glyph.instructions(), [0x2a, 0x2b]);
    assert!(glyph.has_overlapping_components());
}

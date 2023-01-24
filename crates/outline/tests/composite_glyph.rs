//! Semantic decoding of valid composite-glyph records.

use glyphweave_outline::{
    ComponentOffsetMode, ComponentPlacement, ComponentTransform, CompositeGlyph, GlyphBounds,
};
use glyphweave_sfnt::ByteSpan;

fn header() -> [u8; 10] {
    [0xff, 0xff, 0xff, 0xec, 0xff, 0xe2, 0x00, 0x64, 0x00, 0x78]
}

#[test]
fn decodes_an_identity_component_at_an_absolute_span() {
    let mut bytes = vec![0xaa, 0xbb, 0xcc];
    bytes.extend(header());
    bytes.extend([0x00, 0x02, 0x00, 0x2a, 0xfb, 0x07]);
    bytes.push(0xdd);
    let span = ByteSpan::between(3, bytes.len() - 1).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();
    let component = glyph.components()[0];

    assert_eq!(glyph.span(), span);
    assert_eq!(
        glyph.bounds(),
        GlyphBounds::new(-20, -30, 100, 120).unwrap()
    );
    assert_eq!(component.glyph_id().raw(), 42);
    assert_eq!(
        component.placement(),
        ComponentPlacement::Offset {
            x: -5,
            y: 7,
            round_to_grid: false,
            mode: ComponentOffsetMode::Default,
        }
    );
    assert_eq!(component.transform(), ComponentTransform::identity());
    assert!(!component.uses_child_metrics());
    assert!(glyph.instructions().is_empty());
    assert!(!glyph.has_overlapping_components());
}

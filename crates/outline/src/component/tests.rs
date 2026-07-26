use glyphweave_sfnt::GlyphId;

use super::Component;
use crate::{ComponentOffsetMode, ComponentPlacement, ComponentTransform, F2Dot14};

#[test]
fn accessors_preserve_canonical_component_identity() {
    let placement = ComponentPlacement::Offset {
        x: i16::MIN,
        y: i16::MAX,
        round_to_grid: true,
        mode: ComponentOffsetMode::Unscaled,
    };
    let transform = ComponentTransform::Uniform {
        scale: F2Dot14::from_bits(-1),
    };
    let component = Component::new(GlyphId::new(u16::MAX), placement, transform, true);

    assert_eq!(component.glyph_id(), GlyphId::new(u16::MAX));
    assert_eq!(component.placement(), placement);
    assert_eq!(component.transform(), transform);
    assert!(component.uses_child_metrics());
}

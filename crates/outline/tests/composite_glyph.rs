//! Semantic decoding of valid composite-glyph records.

use glyphweave_outline::{
    ComponentOffsetMode, ComponentPlacement, ComponentTransform, CompositeGlyph, GlyphBounds,
};
use glyphweave_sfnt::ByteSpan;

fn header() -> [u8; 10] {
    [0xff, 0xff, 0xff, 0xec, 0xff, 0xe2, 0x00, 0x64, 0x00, 0x78]
}

fn decode_byte_offset(flags: u16) -> CompositeGlyph {
    let mut bytes = header().to_vec();
    bytes.extend(flags.to_be_bytes());
    bytes.extend([0x00, 0x2a, 0x03, 0xfd]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();
    CompositeGlyph::decode(&bytes, span).unwrap()
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

#[test]
fn preserves_full_width_signed_component_offsets() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x03, 0xff, 0xff]);
    bytes.extend(i16::MIN.to_be_bytes());
    bytes.extend(i16::MAX.to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[0].placement(),
        ComponentPlacement::Offset {
            x: i16::MIN,
            y: i16::MAX,
            round_to_grid: false,
            mode: ComponentOffsetMode::Default,
        }
    );
}

#[test]
fn preserves_grid_rounding_for_offset_components() {
    let glyph = decode_byte_offset(0x0006);

    assert_eq!(
        glyph.components()[0].placement(),
        ComponentPlacement::Offset {
            x: 3,
            y: -3,
            round_to_grid: true,
            mode: ComponentOffsetMode::Default,
        }
    );
}

#[test]
fn preserves_scaled_component_offsets() {
    let glyph = decode_byte_offset(0x0802);

    assert_eq!(
        glyph.components()[0].placement(),
        ComponentPlacement::Offset {
            x: 3,
            y: -3,
            round_to_grid: false,
            mode: ComponentOffsetMode::Scaled,
        }
    );
}

#[test]
fn preserves_unscaled_component_offsets() {
    let glyph = decode_byte_offset(0x1002);

    assert_eq!(
        glyph.components()[0].placement(),
        ComponentPlacement::Offset {
            x: 3,
            y: -3,
            round_to_grid: false,
            mode: ComponentOffsetMode::Unscaled,
        }
    );
}

#[test]
fn preserves_component_metric_inheritance() {
    let glyph = decode_byte_offset(0x0202);

    assert!(glyph.components()[0].uses_child_metrics());
}

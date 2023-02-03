//! Semantic decoding of valid composite-glyph records.

use glyphweave_outline::{
    ComponentOffsetMode, ComponentPlacement, ComponentTransform, CompositeGlyph, F2Dot14,
    GlyphBounds,
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

#[test]
fn preserves_first_component_overlap_identity() {
    let glyph = decode_byte_offset(0x0402);

    assert!(glyph.has_overlapping_components());
}

#[test]
fn decodes_uniform_component_scales() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x0a, 0x00, 0x07, 0x00, 0x00, 0x20, 0x00]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[0].transform(),
        ComponentTransform::Uniform {
            scale: F2Dot14::from_bits(0x2000),
        }
    );
}

#[test]
fn decodes_independent_axis_scales() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x42, 0x00, 0x08, 0x00, 0x00]);
    bytes.extend(0x2000_i16.to_be_bytes());
    bytes.extend((-0x1000_i16).to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[0].transform(),
        ComponentTransform::Axis {
            x_scale: F2Dot14::from_bits(0x2000),
            y_scale: F2Dot14::from_bits(-0x1000),
        }
    );
}

#[test]
fn decodes_complete_component_matrices() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x82, 0x00, 0x09, 0x00, 0x00]);
    for value in [0x4000_i16, 0x1000, -0x1000, 0x2000] {
        bytes.extend(value.to_be_bytes());
    }
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[0].transform(),
        ComponentTransform::Matrix {
            x_scale: F2Dot14::ONE,
            scale_01: F2Dot14::from_bits(0x1000),
            scale_10: F2Dot14::from_bits(-0x1000),
            y_scale: F2Dot14::from_bits(0x2000),
        }
    );
}

#[test]
fn decodes_ordered_point_aligned_components() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x22, 0x00, 0x0a, 0x00, 0x00]);
    bytes.extend([0x00, 0x00, 0x00, 0x0b, 0x07, 0x09]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.component_count(), 2);
    assert_eq!(glyph.components()[0].glyph_id().raw(), 10);
    assert_eq!(glyph.components()[1].glyph_id().raw(), 11);
    assert_eq!(
        glyph.components()[1].placement(),
        ComponentPlacement::MatchPoints {
            parent: 7,
            child: 9,
        }
    );
}

#[test]
fn preserves_full_width_point_references() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x22, 0x00, 0x0a, 0x00, 0x00]);
    bytes.extend([0x00, 0x01, 0x00, 0x0b]);
    bytes.extend(u16::MAX.to_be_bytes());
    bytes.extend(0x1234_u16.to_be_bytes());
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[1].placement(),
        ComponentPlacement::MatchPoints {
            parent: u16::MAX,
            child: 0x1234,
        }
    );
}

#[test]
fn ignores_grid_rounding_for_point_alignment() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x22, 0x00, 0x0a, 0x00, 0x00]);
    bytes.extend([0x00, 0x04, 0x00, 0x0b, 0x01, 0x02]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[1].placement(),
        ComponentPlacement::MatchPoints {
            parent: 1,
            child: 2,
        }
    );
}

#[test]
fn ignores_offset_modes_for_point_alignment() {
    let mut bytes = header().to_vec();
    bytes.extend([0x00, 0x22, 0x00, 0x0a, 0x00, 0x00]);
    bytes.extend([0x08, 0x00, 0x00, 0x0b, 0x03, 0x04]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(
        glyph.components()[1].placement(),
        ComponentPlacement::MatchPoints {
            parent: 3,
            child: 4,
        }
    );
}

#[test]
fn preserves_parent_composite_instructions() {
    let mut bytes = header().to_vec();
    bytes.extend([0x01, 0x02, 0x00, 0x0c, 0x00, 0x00]);
    bytes.extend([0x00, 0x03, 0xaa, 0xbb, 0xcc]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.instructions(), [0xaa, 0xbb, 0xcc]);
}

#[test]
fn honors_instruction_flags_on_earlier_components() {
    let mut bytes = header().to_vec();
    bytes.extend([0x01, 0x22, 0x00, 0x0c, 0x00, 0x00]);
    bytes.extend([0x00, 0x02, 0x00, 0x0d, 0x00, 0x00]);
    bytes.extend([0x00, 0x01, 0x2a]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert_eq!(glyph.instructions(), [0x2a]);
}

#[test]
fn accepts_explicit_empty_composite_instructions() {
    let mut bytes = header().to_vec();
    bytes.extend([0x01, 0x02, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00]);
    let span = ByteSpan::sized(0, bytes.len()).unwrap();

    let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

    assert!(glyph.instructions().is_empty());
}

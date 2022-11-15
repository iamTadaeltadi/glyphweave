use glyphweave_sfnt::ByteSpan;

use super::SimpleGlyph;
use crate::{GlyphBounds, Point};

fn fixture() -> SimpleGlyph {
    SimpleGlyph {
        span: ByteSpan::sized(20, 30).unwrap(),
        bounds: GlyphBounds::new(-2, -4, 8, 16).unwrap(),
        contour_ends: vec![2, 3],
        points: vec![
            Point::new(-2, -4, true),
            Point::new(8, 0, false),
            Point::new(0, 16, true),
        ],
        instructions: vec![0xb0, 0x01],
        overlapping_contours: true,
    }
}

#[test]
fn accessors_preserve_validated_source_identity() {
    let glyph = fixture();

    assert_eq!(glyph.span(), ByteSpan::sized(20, 30).unwrap());
    assert_eq!(glyph.bounds(), GlyphBounds::new(-2, -4, 8, 16).unwrap());
    assert_eq!(glyph.contour_count(), 2);
    assert_eq!(glyph.contour_ends(), [2, 3]);
    assert_eq!(glyph.instructions(), [0xb0, 0x01]);
    assert!(glyph.has_overlapping_contours());
}

#[test]
fn contour_lookup_partitions_points_at_exclusive_ends() {
    let glyph = fixture();

    assert_eq!(glyph.contour(0), Some(&glyph.points()[0..2]));
    assert_eq!(glyph.contour(1), Some(&glyph.points()[2..3]));
    assert_eq!(glyph.contour(2), None);
}

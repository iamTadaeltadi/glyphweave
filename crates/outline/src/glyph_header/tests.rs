use super::GlyphHeader;
use crate::GlyphBounds;

const fn bounds() -> GlyphBounds {
    GlyphBounds::new(-1, -2, 3, 4).unwrap()
}

#[test]
fn classifies_every_nonnegative_value_as_simple() {
    for contour_count in [0, 1, i16::MAX] {
        let header = GlyphHeader::new(contour_count, bounds());

        assert_eq!(
            header.simple_contour_count(),
            u16::try_from(contour_count).ok()
        );
        assert!(!header.is_composite());
        assert_eq!(header.number_of_contours(), contour_count);
    }
}

#[test]
fn preserves_every_negative_composite_marker() {
    for marker in [i16::MIN, -2, -1] {
        let header = GlyphHeader::new(marker, bounds());

        assert_eq!(header.simple_contour_count(), None);
        assert!(header.is_composite());
        assert_eq!(header.number_of_contours(), marker);
    }
}

#[test]
fn exposes_the_validated_bounds_unchanged() {
    assert_eq!(GlyphHeader::new(0, bounds()).bounds(), bounds());
}

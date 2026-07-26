use super::GlyphBounds;

#[test]
fn preserves_every_signed_boundary() {
    let bounds = GlyphBounds::new(i16::MIN, -1, i16::MAX, 0).unwrap();

    assert_eq!(bounds.minimum_x(), i16::MIN);
    assert_eq!(bounds.minimum_y(), -1);
    assert_eq!(bounds.maximum_x(), i16::MAX);
    assert_eq!(bounds.maximum_y(), 0);
}

#[test]
fn permits_degenerate_rectangles() {
    let bounds = GlyphBounds::new(7, -3, 7, -3).unwrap();

    assert_eq!(bounds.minimum_x(), bounds.maximum_x());
    assert_eq!(bounds.minimum_y(), bounds.maximum_y());
}

#[test]
fn rejects_an_inverted_axis_independently() {
    assert_eq!(GlyphBounds::new(1, 0, 0, 1), None);
    assert_eq!(GlyphBounds::new(0, 1, 1, 0), None);
    assert_eq!(GlyphBounds::new(1, 1, 0, 0), None);
}

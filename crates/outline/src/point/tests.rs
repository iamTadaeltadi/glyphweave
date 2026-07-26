use super::Point;

#[test]
fn preserves_signed_coordinate_extremes() {
    let point = Point::new(i16::MIN, i16::MAX, true);

    assert_eq!(point.x(), i16::MIN);
    assert_eq!(point.y(), i16::MAX);
    assert!(point.is_on_curve());
}

#[test]
fn preserves_off_curve_identity_independently_of_position() {
    let point = Point::new(0, 0, false);

    assert!(!point.is_on_curve());
    assert_eq!((point.x(), point.y()), (0, 0));
}

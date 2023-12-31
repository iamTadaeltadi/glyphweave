use super::ValueRecord;

#[test]
fn exposes_each_adjustment() {
    let value = ValueRecord::new(1, -2, 3, -4);
    assert_eq!(value.x_placement(), 1);
    assert_eq!(value.y_placement(), -2);
    assert_eq!(value.x_advance(), 3);
    assert_eq!(value.y_advance(), -4);
    assert!(!value.is_zero());
}

#[test]
fn default_is_zero() {
    assert!(ValueRecord::default().is_zero());
    assert_eq!(ValueRecord::default(), ValueRecord::new(0, 0, 0, 0));
}

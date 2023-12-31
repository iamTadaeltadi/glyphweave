use super::ValueFormat;

#[test]
fn rejects_reserved_bits() {
    assert!(ValueFormat::from_bits(0x0100).is_none());
    assert!(ValueFormat::from_bits(0xff00).is_none());
    assert!(ValueFormat::from_bits(0x00ff).is_some());
}

#[test]
fn reports_present_fields_and_size() {
    let format = ValueFormat::from_bits(0x000f).unwrap();
    assert!(format.has_x_placement());
    assert!(format.has_y_placement());
    assert!(format.has_x_advance());
    assert!(format.has_y_advance());
    assert!(!format.has_any_device());
    assert_eq!(format.byte_size(), 8);
}

#[test]
fn detects_device_tables() {
    let format = ValueFormat::from_bits(0x0011).unwrap();
    assert!(format.has_x_placement());
    assert!(format.has_any_device());
    assert_eq!(format.byte_size(), 4);
}

#[test]
fn empty_format_encodes_nothing() {
    let format = ValueFormat::default();
    assert!(format.is_empty());
    assert_eq!(format.byte_size(), 0);
    assert!(!format.has_any_device());
}

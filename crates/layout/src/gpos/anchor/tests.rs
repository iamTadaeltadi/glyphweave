use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTarget};

use super::{Anchor, AnchorKind};

fn decode(bytes: &[u8]) -> glyphweave_sfnt::Decoded<Anchor> {
    Anchor::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
}

#[test]
fn constructors_expose_each_shape() {
    let coordinate = Anchor::coordinate(1, -2);
    assert_eq!((coordinate.x(), coordinate.y()), (1, -2));
    assert_eq!(coordinate.kind(), AnchorKind::Coordinate);
    assert_eq!(coordinate.contour_point_index(), None);
    assert!(!coordinate.has_devices());

    let contour = Anchor::contour_point(3, 4, 9);
    assert_eq!(contour.contour_point_index(), Some(9));
    assert_eq!(contour.kind(), AnchorKind::ContourPoint(9));
    assert!(!contour.has_devices());

    let empty_devices = Anchor::devices(5, 6, None, None);
    assert!(!empty_devices.has_devices());
    assert_eq!(empty_devices.contour_point_index(), None);

    let devices = Anchor::devices(5, 6, Some(10), None);
    assert!(devices.has_devices());
    assert_eq!(
        devices.kind(),
        AnchorKind::Devices {
            x_device: Some(10),
            y_device: None,
        }
    );
}

#[test]
fn decodes_format_1_coordinate() {
    let anchor = decode(&[0x00, 0x01, 0x01, 0x2c, 0xff, 0xce]).unwrap();
    assert_eq!((anchor.x(), anchor.y()), (300, -50));
    assert_eq!(anchor.kind(), AnchorKind::Coordinate);
}

#[test]
fn decodes_format_2_contour_point() {
    let anchor = decode(&[0x00, 0x02, 0x01, 0x2c, 0xff, 0xce, 0x00, 0x05]).unwrap();
    assert_eq!((anchor.x(), anchor.y()), (300, -50));
    assert_eq!(anchor.contour_point_index(), Some(5));
}

#[test]
fn decodes_format_3_with_null_devices() {
    let anchor = decode(&[0x00, 0x03, 0x01, 0x2c, 0xff, 0xce, 0x00, 0x00, 0x00, 0x00]).unwrap();
    assert_eq!((anchor.x(), anchor.y()), (300, -50));
    assert!(!anchor.has_devices());
    assert_eq!(
        anchor.kind(),
        AnchorKind::Devices {
            x_device: None,
            y_device: None,
        }
    );
}

#[test]
fn decodes_format_3_with_bounded_device_offsets() {
    // The header is ten bytes; the x device offset points at the first trailing byte.
    let bytes = [
        0x00, 0x03, 0x01, 0x2c, 0xff, 0xce, 0x00, 0x0a, 0x00, 0x00, 0xde, 0xad,
    ];
    let anchor = decode(&bytes).unwrap();
    assert!(anchor.has_devices());
    assert_eq!(
        anchor.kind(),
        AnchorKind::Devices {
            x_device: Some(10),
            y_device: None,
        }
    );
}

#[test]
fn rejects_unsupported_anchor_format() {
    let diagnostic = decode(&[0x00, 0x04, 0x00, 0x00]).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGposAnchorFormat { format: 4 })
    );
    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
}

#[test]
fn rejects_device_offset_inside_the_header() {
    let bytes = [
        0x00, 0x03, 0x01, 0x2c, 0xff, 0xce, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00,
    ];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::GposAnchorDevice,
            offset: 4,
            minimum: 10,
            container_length: 12,
        })
    );
}

#[test]
fn rejects_device_offset_past_the_owner() {
    let bytes = [0x00, 0x03, 0x01, 0x2c, 0xff, 0xce, 0x00, 0x64, 0x00, 0x00];
    let diagnostic = decode(&bytes).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::GposAnchorDevice,
            offset: 100,
            minimum: 10,
            container_length: 10,
        })
    );
}

#[test]
fn reports_truncation_before_the_coordinate() {
    let diagnostic = decode(&[0x00, 0x01, 0x01]).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::UnexpectedEnd {
            needed: 2,
            available: 1,
        }
    );
}

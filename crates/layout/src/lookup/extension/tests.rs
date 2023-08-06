use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTableKind, LayoutTarget};

use super::{decode_extension, flatten_extensions};

fn decode(
    bytes: &[u8],
    table: LayoutTableKind,
) -> glyphweave_sfnt::Decoded<super::DecodedExtension> {
    decode_extension(bytes, ByteSpan::between(0, bytes.len()).unwrap(), table)
}

#[test]
fn resolves_absolute_extension_payload() {
    let bytes = [
        0xaa, 0xbb, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x08, 0xfe,
    ];
    let extension = decode_extension(
        &bytes,
        ByteSpan::between(2, bytes.len()).unwrap(),
        LayoutTableKind::Substitution,
    )
    .unwrap();
    assert_eq!(extension.lookup_type, 4);
    assert_eq!(extension.type_position, 4);
    assert_eq!(extension.payload, ByteSpan::between(10, 11).unwrap());
}

#[test]
fn rejects_unknown_extension_format() {
    let diagnostic = decode(&[0x00, 0x02], LayoutTableKind::Positioning).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedExtensionFormat {
            table: LayoutTableKind::Positioning,
            format: 2,
        })
    );
}

#[test]
fn rejects_recursive_extension_type() {
    let diagnostic = decode(&[0x00, 0x01, 0x00, 0x07], LayoutTableKind::Substitution).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::RecursiveExtensionLookupType {
            table: LayoutTableKind::Substitution,
            lookup_type: 7,
        })
    );
}

#[test]
fn rejects_reserved_lookup_type() {
    let diagnostic = decode(&[0x00, 0x01, 0x00, 0x0a], LayoutTableKind::Positioning).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLookupType {
            table: LayoutTableKind::Positioning,
            lookup_type: 10,
        })
    );
}

#[test]
fn rejects_extension_target_inside_wrapper() {
    let bytes = [0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04];
    let diagnostic = decode(&bytes, LayoutTableKind::Substitution).unwrap_err();
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::ExtensionSubtable,
            offset: 4,
            minimum: 8,
            container_length: 8,
        })
    );
}

#[test]
fn preserves_full_width_invalid_extension_offset() {
    let bytes = [0x00, 0x01, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff];
    let diagnostic = decode(&bytes, LayoutTableKind::Substitution).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(4, 8));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::ExtensionSubtable,
            offset: u32::MAX,
            minimum: 8,
            container_length: 8,
        })
    );
}

#[test]
fn flattens_same_typed_wrappers_in_order() {
    let bytes = [
        0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x10, // wrapper one
        0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x0a, // wrapper two
        0xaa, 0xbb, 0xcc, 0xdd,
    ];
    let wrappers = [
        ByteSpan::between(0, 20).unwrap(),
        ByteSpan::between(8, 20).unwrap(),
    ];
    let (lookup_type, payloads) =
        flatten_extensions(&bytes, wrappers.into(), LayoutTableKind::Substitution).unwrap();

    assert_eq!(lookup_type, 4);
    assert_eq!(
        payloads,
        [
            ByteSpan::between(16, 20).unwrap(),
            ByteSpan::between(18, 20).unwrap()
        ]
    );
}

#[test]
fn rejects_different_real_extension_types() {
    let bytes = [
        0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x10, // type four
        0x00, 0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0x0a, // type five
        0xaa, 0xbb, 0xcc, 0xdd,
    ];
    let wrappers = [
        ByteSpan::between(0, 20).unwrap(),
        ByteSpan::between(8, 20).unwrap(),
    ];
    let diagnostic =
        flatten_extensions(&bytes, wrappers.into(), LayoutTableKind::Substitution).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(10, 12));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedExtensionLookupTypes {
            table: LayoutTableKind::Substitution,
            expected: 4,
            actual: 5,
        })
    );
}

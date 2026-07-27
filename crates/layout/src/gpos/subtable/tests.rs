use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTableKind};

use super::GposSubtable;
use crate::ValueRecord;

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

const fn whole(bytes: &[u8]) -> ByteSpan {
    ByteSpan::sized(0, bytes.len()).expect("subtable span fits the buffer")
}

fn cause(bytes: &[u8], lookup_type: u16) -> LayoutCause {
    match GposSubtable::decode(bytes, whole(bytes), lookup_type)
        .expect_err("decode should fail")
        .cause()
    {
        Cause::Layout(cause) => cause,
        other => panic!("expected a layout cause, found {other:?}"),
    }
}

/// A `SinglePos` format 1 subtable covering one glyph with an x-advance adjustment.
fn single_positioning_subtable(bytes: &mut Vec<u8>) {
    push_u16(bytes, 1); // SinglePos posFormat
    push_u16(bytes, 8); // coverageOffset (relative to the subtable start)
    push_u16(bytes, 0x0004); // valueFormat: x advance
    push_i16(bytes, -30); // shared valueRecord.xAdvance
    push_u16(bytes, 1); // Coverage format 1
    push_u16(bytes, 1); // glyph count
    push_u16(bytes, 7); // glyph id 7
}

#[test]
fn unwraps_a_type_9_extension_into_its_real_subtable() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // ExtensionPos posFormat
    push_u16(&mut bytes, 1); // extensionLookupType: single positioning
    push_u32(&mut bytes, 8); // extensionOffset (relative to the wrapper start)
    single_positioning_subtable(&mut bytes);

    let subtable = GposSubtable::decode(&bytes, whole(&bytes), 9).expect("valid extension");
    assert_eq!(subtable.lookup_type(), 1);
    let GposSubtable::Single(single) = subtable else {
        panic!("expected a single-positioning subtable");
    };
    assert_eq!(
        single.adjustment(GlyphId::new(7)),
        Some(ValueRecord::new(0, 0, -30, 0)),
    );
}

#[test]
fn rejects_a_recursive_extension_type() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // posFormat
    push_u16(&mut bytes, 9); // extensionLookupType names the transport type itself
    assert_eq!(
        cause(&bytes, 9),
        LayoutCause::RecursiveGposExtension { lookup_type: 9 }
    );
}

#[test]
fn rejects_an_out_of_range_extension_type() {
    for extension_type in [0, 10, 42] {
        let mut bytes = Vec::new();
        push_u16(&mut bytes, 1); // posFormat
        push_u16(&mut bytes, extension_type);
        assert_eq!(
            cause(&bytes, 9),
            LayoutCause::UnsupportedGposExtensionType {
                lookup_type: extension_type,
            }
        );
    }
}

#[test]
fn rejects_an_unknown_extension_format() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 2); // unsupported ExtensionPos format
    assert_eq!(
        cause(&bytes, 9),
        LayoutCause::UnsupportedExtensionFormat {
            table: LayoutTableKind::Positioning,
            format: 2,
        }
    );
}

#[test]
fn rejects_an_unknown_lookup_type() {
    let bytes = [0u8; 8];
    for lookup_type in [0, 10, 11] {
        assert_eq!(
            cause(&bytes, lookup_type),
            LayoutCause::InvalidLookupType {
                table: LayoutTableKind::Positioning,
                lookup_type,
            }
        );
    }
}

#[test]
fn renders_the_new_extension_causes() {
    assert_eq!(
        LayoutCause::RecursiveGposExtension { lookup_type: 9 }.to_string(),
        "GPOS extension wrapper recursively targets extension lookup type 9"
    );
    assert_eq!(
        LayoutCause::UnsupportedGposExtensionType { lookup_type: 10 }.to_string(),
        "GPOS extension wrapper targets unsupported lookup type 10"
    );
}

use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::GposLookupList;
use crate::{GposSubtable, ValueRecord};

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
    ByteSpan::sized(0, bytes.len()).expect("lookup list span fits the buffer")
}

fn cause(bytes: &[u8]) -> LayoutCause {
    match GposLookupList::decode(bytes, whole(bytes))
        .expect_err("decode should fail")
        .cause()
    {
        Cause::Layout(cause) => cause,
        other => panic!("expected a layout cause, found {other:?}"),
    }
}

/// Asserts a Lookup holds a single-positioning subtable adjusting `glyph` by `x_advance`.
fn assert_single_advance(subtable: &GposSubtable, glyph: u16, x_advance: i16) {
    let GposSubtable::Single(single) = subtable else {
        panic!(
            "expected a single-positioning subtable, found type {}",
            subtable.lookup_type()
        );
    };
    assert_eq!(
        single.adjustment(GlyphId::new(glyph)),
        Some(ValueRecord::new(0, 0, x_advance, 0)),
    );
}

/// A two-Lookup GPOS `LookupList`: an `ExtensionPos` wrapper over `SinglePos`, then a direct
/// `SinglePos`. Every offset is relative to the structure that stores it.
fn two_lookup_list() -> Vec<u8> {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 2); // lookupCount
    push_u16(&mut bytes, 6); // lookupOffset[0]
    push_u16(&mut bytes, 36); // lookupOffset[1]
    // Lookup 0: extension positioning.
    push_u16(&mut bytes, 9); // lookupType
    push_u16(&mut bytes, 0); // lookupFlag
    push_u16(&mut bytes, 1); // subtableCount
    push_u16(&mut bytes, 8); // subtableOffset[0]
    push_u16(&mut bytes, 1); // ExtensionPos posFormat
    push_u16(&mut bytes, 1); // extensionLookupType: single positioning
    push_u32(&mut bytes, 8); // extensionOffset
    push_u16(&mut bytes, 1); // SinglePos posFormat
    push_u16(&mut bytes, 8); // coverageOffset
    push_u16(&mut bytes, 0x0004); // valueFormat: x advance
    push_i16(&mut bytes, -30); // shared valueRecord.xAdvance
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 1); // glyph count
    push_u16(&mut bytes, 7); // glyph id 7
    // Lookup 1: direct single positioning.
    push_u16(&mut bytes, 1); // lookupType
    push_u16(&mut bytes, 0); // lookupFlag
    push_u16(&mut bytes, 1); // subtableCount
    push_u16(&mut bytes, 8); // subtableOffset[0]
    push_u16(&mut bytes, 1); // SinglePos posFormat
    push_u16(&mut bytes, 8); // coverageOffset
    push_u16(&mut bytes, 0x0004); // valueFormat: x advance
    push_i16(&mut bytes, -20); // shared valueRecord.xAdvance
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 1); // glyph count
    push_u16(&mut bytes, 5); // glyph id 5
    bytes
}

#[test]
fn decodes_lookups_in_index_order_and_flattens_extensions() {
    let bytes = two_lookup_list();
    let list = GposLookupList::decode(&bytes, whole(&bytes)).expect("valid lookup list");

    assert_eq!(list.len(), 2);
    assert_eq!(list.lookups().len(), 2);

    let extension = list.lookup(0).expect("index zero present");
    assert_eq!(extension.lookup_type(), 1); // the wrapped type, not the type 9 transport
    assert_eq!(extension.len(), 1);
    assert_single_advance(&extension.subtables()[0], 7, -30);

    let direct = list.lookup(1).expect("index one present");
    assert_eq!(direct.lookup_type(), 1);
    assert_single_advance(&direct.subtables()[0], 5, -20);

    assert!(list.lookup(2).is_none());
}

#[test]
fn decodes_an_empty_lookup_list() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 0); // lookupCount
    let list = GposLookupList::decode(&bytes, whole(&bytes)).expect("empty list decodes");
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert!(list.lookup(0).is_none());
}

#[test]
fn rejects_a_lookup_offset_inside_the_header() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // lookupCount
    push_u16(&mut bytes, 0); // lookupOffset[0] points back into the header
    assert_eq!(
        cause(&bytes),
        LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::Lookup,
            offset: 0,
            minimum: 4,
            container_length: 4,
        }
    );
}

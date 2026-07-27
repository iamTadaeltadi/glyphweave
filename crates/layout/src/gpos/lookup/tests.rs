use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTableKind, LayoutTarget};

use super::GposLookup;
use crate::MarkFilter;

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

fn push_i16(bytes: &mut Vec<u8>, value: i16) {
    bytes.extend_from_slice(&value.to_be_bytes());
}

const fn whole(bytes: &[u8]) -> ByteSpan {
    ByteSpan::sized(0, bytes.len()).expect("lookup span fits the buffer")
}

fn cause(bytes: &[u8]) -> LayoutCause {
    match GposLookup::decode(bytes, whole(bytes))
        .expect_err("decode should fail")
        .cause()
    {
        Cause::Layout(cause) => cause,
        other => panic!("expected a layout cause, found {other:?}"),
    }
}

#[test]
fn rejects_reserved_lookup_flags() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // lookupType
    push_u16(&mut bytes, 0x00e0); // reserved bits set
    assert_eq!(
        cause(&bytes),
        LayoutCause::ReservedLookupFlags { flags: 0x00e0 }
    );
}

#[test]
fn rejects_an_empty_subtable_list() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 2); // lookupType
    push_u16(&mut bytes, 0); // lookupFlag
    push_u16(&mut bytes, 0); // subtableCount
    assert_eq!(
        cause(&bytes),
        LayoutCause::EmptyLookupSubtables {
            table: LayoutTableKind::Positioning,
            lookup_type: 2,
        }
    );
}

#[test]
fn rejects_a_subtable_offset_inside_the_header() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // lookupType
    push_u16(&mut bytes, 0); // lookupFlag
    push_u16(&mut bytes, 1); // subtableCount
    push_u16(&mut bytes, 0); // subtableOffset[0] points back into the header
    assert_eq!(
        cause(&bytes),
        LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::LookupSubtable,
            offset: 0,
            minimum: 8,
            container_length: 8,
        }
    );
}

#[test]
fn reads_a_mark_filtering_set_before_resolving_subtables() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 1); // lookupType: single positioning
    push_u16(&mut bytes, 0x0010); // USE_MARK_FILTERING_SET
    push_u16(&mut bytes, 1); // subtableCount
    push_u16(&mut bytes, 10); // subtableOffset[0]
    push_u16(&mut bytes, 3); // markFilteringSet index
    push_u16(&mut bytes, 1); // SinglePos posFormat 1
    push_u16(&mut bytes, 8); // coverageOffset
    push_u16(&mut bytes, 0x0004); // valueFormat: x advance
    push_i16(&mut bytes, -20); // shared valueRecord.xAdvance
    push_u16(&mut bytes, 1); // Coverage format 1
    push_u16(&mut bytes, 1); // glyph count
    push_u16(&mut bytes, 5); // glyph id 5

    let lookup = GposLookup::decode(&bytes, whole(&bytes)).expect("valid lookup");
    assert_eq!(lookup.lookup_type(), 1);
    assert_eq!(lookup.len(), 1);
    assert!(!lookup.is_empty());
    assert_eq!(lookup.flags().mark_filter(), MarkFilter::Set(3));
    assert_eq!(lookup.subtables()[0].lookup_type(), 1);
}

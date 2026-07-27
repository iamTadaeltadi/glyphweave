use glyphweave_sfnt::ByteSpan;

use super::RetainedGpos;
use crate::GposLookupList;

fn sample_lookup_list() -> GposLookupList {
    let bytes = [
        0x00, 0x01, // lookup count
        0x00, 0x04, // lookup offset[0]
        // Lookup @4
        0x00, 0x01, // lookupType: single positioning
        0x00, 0x00, // lookupFlag
        0x00, 0x01, // subtableCount
        0x00, 0x08, // subtableOffset[0]
        // SinglePos @12
        0x00, 0x01, // posFormat 1
        0x00, 0x08, // coverageOffset
        0x00, 0x04, // valueFormat: x advance
        0xff, 0xec, // xAdvance = -20
        // Coverage @20
        0x00, 0x01, // Coverage format 1
        0x00, 0x01, // glyph count
        0x00, 0x05, // glyph id 5
    ];
    let span = ByteSpan::sized(0, bytes.len()).expect("span fits the buffer");
    GposLookupList::decode(&bytes, span).expect("valid gpos lookup list")
}

#[test]
fn keeps_reachable_lookup_and_empties_the_unreachable_slot() {
    let list = sample_lookup_list();
    assert_eq!(list.len(), 1);

    let kept = list.retain(|index| index == 0);
    assert_eq!(kept.len(), 1);
    let lookup = kept.lookup(0).expect("slot 0 present");
    assert_eq!(lookup.lookup_type(), 1);
    assert_eq!(lookup.len(), 1);

    let dropped = list.retain(|_| false);
    assert_eq!(dropped.len(), 1); // the slot is preserved
    let placeholder = dropped.lookup(0).expect("slot 0 preserved");
    assert!(placeholder.is_empty()); // but its subtables are gone
    assert_eq!(placeholder.lookup_type(), 1); // type and flags survive
    assert_eq!(dropped.lookup_list().len(), 1);
}

#[test]
fn default_retained_gpos_is_empty() {
    let positioning = RetainedGpos::default();
    assert!(positioning.is_empty());
    assert_eq!(positioning.len(), 0);
    assert!(positioning.lookup(0).is_none());
}

use glyphweave_sfnt::{ByteSpan, LayoutTableKind};

use super::Lookup;
use crate::LookupFlags;

#[test]
fn preserves_table_type_flags_and_subtable_order() {
    let first = ByteSpan::between(20, 30).unwrap();
    let second = ByteSpan::between(10, 30).unwrap();
    let lookup = Lookup::new(
        LayoutTableKind::Substitution,
        4,
        LookupFlags::default(),
        [first, second],
    );

    assert_eq!(lookup.table(), LayoutTableKind::Substitution);
    assert_eq!(lookup.lookup_type(), 4);
    assert_eq!(lookup.flags(), LookupFlags::default());
    assert_eq!(lookup.subtables(), &[first, second]);
}

#[test]
fn empty_manual_lookup_reports_empty() {
    let lookup = Lookup::new(LayoutTableKind::Positioning, 1, LookupFlags::default(), []);
    assert!(lookup.is_empty());
    assert_eq!(lookup.len(), 0);
}

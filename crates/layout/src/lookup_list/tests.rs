use glyphweave_sfnt::{ByteSpan, LayoutTableKind};

use super::LookupList;
use crate::{Lookup, LookupFlags};

fn lookup(lookup_type: u16) -> Lookup {
    Lookup::new(
        LayoutTableKind::Substitution,
        lookup_type,
        LookupFlags::default(),
        [ByteSpan::between(10, 12).unwrap()],
    )
}

#[test]
fn preserves_lookup_index_order() {
    let list = LookupList::new([lookup(4), lookup(1)]);
    assert_eq!(list.lookup(0).unwrap().lookup_type(), 4);
    assert_eq!(list.lookup(1).unwrap().lookup_type(), 1);
    assert_eq!(list.lookup(2), None);
    assert_eq!(list.table(), Some(LayoutTableKind::Substitution));
}

#[test]
fn empty_list_has_no_table_family() {
    let list = LookupList::default();
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.table(), None);
}

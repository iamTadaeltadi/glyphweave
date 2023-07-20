use super::LookupFlags;
use crate::MarkFilter;

#[test]
fn default_flags_apply_no_qualifiers() {
    let flags = LookupFlags::default();
    assert!(!flags.right_to_left());
    assert!(!flags.ignores_base_glyphs());
    assert!(!flags.ignores_ligatures());
    assert_eq!(flags.mark_filter(), MarkFilter::None);
}

#[test]
fn qualifiers_preserve_independent_meanings() {
    let flags = LookupFlags::new(true, true, true, MarkFilter::Set(9));
    assert!(flags.right_to_left());
    assert!(flags.ignores_base_glyphs());
    assert!(flags.ignores_ligatures());
    assert_eq!(flags.mark_filter(), MarkFilter::Set(9));
}

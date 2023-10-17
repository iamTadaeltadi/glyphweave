use super::ClassDefinition;
use glyphweave_sfnt::GlyphId;

#[test]
fn default_definition_is_implicit_class_zero() {
    let classes: ClassDefinition = ClassDefinition::default();
    assert!(classes.is_empty());
    assert_eq!(classes.len(), 0);
    assert_eq!(classes.highest_class(), 0);
    assert_eq!(classes.class_of(GlyphId::new(17)), 0);
}

#[test]
fn assignments_are_iterated_in_glyph_order() {
    let classes = ClassDefinition::from_assignments([(GlyphId::new(9), 1), (GlyphId::new(2), 3)]);
    assert_eq!(
        classes.assignments().collect::<Vec<_>>(),
        [(GlyphId::new(2), 3), (GlyphId::new(9), 1)]
    );
    assert_eq!(classes.highest_class(), 3);
}

#[test]
fn later_assignments_replace_or_remove_a_glyph() {
    let classes = ClassDefinition::from_assignments([
        (GlyphId::new(4), 1),
        (GlyphId::new(4), 7),
        (GlyphId::new(4), 0),
    ]);
    assert!(classes.is_empty());
}

#[test]
fn iterator_collection_is_canonical() {
    let classes: ClassDefinition = [(GlyphId::new(6), 2), (GlyphId::new(3), 4)]
        .into_iter()
        .collect();
    assert_eq!(classes.len(), 2);
    assert_eq!(classes.class_of(GlyphId::new(3)), 4);
}

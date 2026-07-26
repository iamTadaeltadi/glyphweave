use super::Feature;

#[test]
fn default_feature_is_empty() {
    assert!(Feature::default().is_empty());
}

#[test]
fn lookup_indices_are_sorted_and_deduplicated() {
    let feature = Feature::new(None, [8, 3, 8, 1]);
    assert_eq!(feature.lookup_indices(), &[1, 3, 8]);
}

#[test]
fn parameter_position_is_preserved_independently() {
    let feature = Feature::new(Some(42), []);
    assert_eq!(feature.parameters_position(), Some(42));
    assert!(!feature.is_empty());
}

use super::LanguageSystem;

#[test]
fn default_language_system_selects_nothing() {
    let language = LanguageSystem::default();
    assert!(language.is_empty());
    assert_eq!(language.required_feature(), None);
    assert!(language.feature_indices().is_empty());
}

#[test]
fn optional_features_are_sorted_and_deduplicated() {
    let language = LanguageSystem::new(None, [9, 2, 9, 4]);
    assert_eq!(language.feature_indices(), &[2, 4, 9]);
    assert!(!language.is_empty());
}

#[test]
fn required_feature_is_independent_of_optional_set() {
    let language = LanguageSystem::new(Some(7), [7, 2]);
    assert_eq!(language.required_feature(), Some(7));
    assert_eq!(language.feature_indices(), &[2, 7]);
}

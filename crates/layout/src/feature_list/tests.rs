use glyphweave_sfnt::Tag;

use super::FeatureList;
use crate::Feature;

#[test]
fn default_feature_list_is_empty() {
    let features = FeatureList::default();
    assert!(features.is_empty());
    assert_eq!(features.len(), 0);
    assert_eq!(features.feature(0), None);
    assert_eq!(features.tag(0), None);
}

#[test]
fn records_receive_stable_tag_order() {
    let first_liga = Feature::new(None, [1]);
    let second_liga = Feature::new(None, [2]);
    let features = FeatureList::new([
        (Tag::new(*b"liga"), first_liga.clone()),
        (Tag::new(*b"kern"), Feature::default()),
        (Tag::new(*b"liga"), second_liga.clone()),
    ]);
    assert_eq!(features.tag(0), Some(Tag::new(*b"kern")));
    assert_eq!(features.feature(1), Some(&first_liga));
    assert_eq!(features.feature(2), Some(&second_liga));
}

#[test]
fn equal_tags_remain_distinct_indexed_records() {
    let features = FeatureList::new([
        (Tag::new(*b"kern"), Feature::new(None, [4])),
        (Tag::new(*b"kern"), Feature::new(None, [9])),
    ]);
    let indices: Vec<_> = features
        .features_with_tag(Tag::new(*b"kern"))
        .map(|(index, _)| index)
        .collect();
    assert_eq!(indices, [0, 1]);
}

#[test]
fn absent_tag_has_no_matching_features() {
    let features = FeatureList::new([(Tag::new(*b"kern"), Feature::default())]);
    assert_eq!(features.features().len(), 1);
    assert_eq!(features.features_with_tag(Tag::new(*b"liga")).count(), 0);
}

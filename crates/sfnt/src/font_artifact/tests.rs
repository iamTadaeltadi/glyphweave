use std::sync::Arc;

use super::FontArtifact;

#[test]
fn owns_and_exposes_only_immutable_bytes() {
    let artifact = FontArtifact::new(vec![0, 1, 2, 3]);
    assert_eq!(artifact.len(), 4);
    assert!(!artifact.is_empty());
    assert_eq!(artifact.as_bytes(), [0, 1, 2, 3]);
    assert_eq!(artifact.as_ref(), [0, 1, 2, 3]);
}

#[test]
fn clones_share_the_same_allocation() {
    let artifact = FontArtifact::new(vec![7; 4096]);
    let cloned = artifact.clone();
    assert!(Arc::ptr_eq(&artifact.bytes, &cloned.bytes));
    assert_eq!(artifact, cloned);
}

#[test]
fn empty_and_debug_behavior_is_bounded() {
    let artifact = FontArtifact::default();
    assert!(artifact.is_empty());
    assert_eq!(format!("{artifact:?}"), "FontArtifact { length: 0, .. }");
}

#[test]
fn construction_from_a_borrowed_slice_copies_ownership() {
    let mut source = vec![1, 2, 3];
    let artifact = FontArtifact::new(source.as_slice());
    source[0] = 9;
    assert_eq!(artifact.as_bytes(), [1, 2, 3]);
    assert_eq!(source, [9, 2, 3]);
}

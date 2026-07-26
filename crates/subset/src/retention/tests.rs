use super::Retention;

#[test]
fn drop_is_the_safe_default() {
    let retention = Retention::default();
    assert_eq!(retention, Retention::Drop);
    assert!(!retention.is_retained());
    assert!(!retention.requires_rewrite());
    assert!(!retention.keeps_verbatim());
}

#[test]
fn keep_preserves_original_bytes() {
    assert!(Retention::Keep.is_retained());
    assert!(Retention::Keep.keeps_verbatim());
    assert!(!Retention::Keep.requires_rewrite());
}

#[test]
fn rewrite_retains_transformed_bytes() {
    assert!(Retention::Rewrite.is_retained());
    assert!(Retention::Rewrite.requires_rewrite());
    assert!(!Retention::Rewrite.keeps_verbatim());
}

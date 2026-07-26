use super::CodepointSet;

#[test]
fn new_set_is_empty() {
    let codepoints = CodepointSet::new();
    assert_eq!(codepoints, CodepointSet::default());
    assert_eq!(codepoints.len(), 0);
    assert!(codepoints.is_empty());
    assert!(!codepoints.contains('A'));
}

#[test]
fn borrowed_iteration_is_exact_and_double_ended() {
    let codepoints = CodepointSet::new();
    let mut iter = (&codepoints).into_iter();
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next_back(), None);
    assert_eq!(codepoints.iter().next(), None);
}

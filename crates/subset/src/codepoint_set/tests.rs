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

#[test]
fn insertion_and_removal_report_membership_changes() {
    let mut codepoints = CodepointSet::new();
    assert!(codepoints.insert('🦀'));
    assert!(!codepoints.insert('🦀'));
    assert!(codepoints.contains('🦀'));
    assert!(codepoints.remove('🦀'));
    assert!(!codepoints.remove('🦀'));
    assert!(codepoints.is_empty());
}

#[test]
fn retention_filters_requested_scalars() {
    let mut codepoints = CodepointSet::new();
    for codepoint in ['A', 'β', '中', '🦀'] {
        codepoints.insert(codepoint);
    }
    codepoints.retain(|codepoint| codepoint.is_ascii());
    assert_eq!(codepoints.iter().collect::<Vec<_>>(), ['A']);
}

#[test]
fn collection_deduplicates_and_orders_unicode_scalars() {
    let mut codepoints: CodepointSet = ['🦀', 'A', 'β', 'A'].into_iter().collect();
    codepoints.extend(['中', 'β']);
    assert_eq!(
        codepoints.iter().collect::<Vec<_>>(),
        ['A', 'β', '中', '🦀']
    );
}

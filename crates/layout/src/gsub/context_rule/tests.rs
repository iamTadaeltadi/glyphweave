use glyphweave_sfnt::GlyphId;

use super::ContextRule;
use crate::SubstitutionRecord;

#[test]
fn non_chained_rule_keeps_a_nonempty_input_and_actions() {
    let rule = ContextRule::new(
        GlyphId::new(3),
        [GlyphId::new(4), GlyphId::new(5)],
        [SubstitutionRecord::new(1, 9)],
    );
    assert!(rule.backtrack().is_empty());
    assert_eq!(
        rule.input(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5),]
    );
    assert!(rule.lookahead().is_empty());
    assert_eq!(rule.records(), [SubstitutionRecord::new(1, 9)]);
}

#[test]
fn chained_rule_preserves_all_three_sequence_directions() {
    let rule = ContextRule::chained(
        [2_u16, 1],
        3,
        [4, 5],
        [6, 7],
        [
            SubstitutionRecord::new(2, 10),
            SubstitutionRecord::new(0, 8),
        ],
    );
    assert_eq!(rule.backtrack(), [2, 1]);
    assert_eq!(rule.input(), [3, 4, 5]);
    assert_eq!(rule.lookahead(), [6, 7]);
    assert_eq!(
        rule.records(),
        [
            SubstitutionRecord::new(2, 10),
            SubstitutionRecord::new(0, 8),
        ]
    );
}

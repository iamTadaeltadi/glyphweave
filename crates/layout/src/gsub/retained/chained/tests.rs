use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::retain;
use crate::{
    ChainedContextSubstitution, ClassDefinition, ContextRule, Coverage, GsubClosure,
    SubstitutionRecord,
};

#[test]
fn coverage_rule_retention_narrows_every_sequence_direction() {
    let input = ChainedContextSubstitution::CoverageRule(ContextRule::chained(
        [Coverage::from_glyphs([GlyphId::new(1), GlyphId::new(8)])],
        Coverage::from_glyphs([GlyphId::new(3), GlyphId::new(9)]),
        [],
        [Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(10)])],
        [SubstitutionRecord::new(0, 7)],
    ));

    let Some(ChainedContextSubstitution::CoverageRule(retained)) =
        retain(&input, &closed([1, 3, 4]))
    else {
        panic!("feasible chained Coverage rule must remain");
    };

    assert_eq!(retained.backtrack()[0].glyphs(), [GlyphId::new(1)]);
    assert_eq!(retained.input()[0].glyphs(), [GlyphId::new(3)]);
    assert_eq!(retained.lookahead()[0].glyphs(), [GlyphId::new(4)]);
    assert_eq!(retained.records(), [SubstitutionRecord::new(0, 7)]);
}

#[test]
fn class_rule_retention_prunes_domains_and_infeasible_rules() {
    let input = ChainedContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3), GlyphId::new(8)]),
        backtrack_classes: ClassDefinition::from_assignments([(GlyphId::new(1), 1)]),
        input_classes: ClassDefinition::from_assignments([(GlyphId::new(3), 2)]),
        lookahead_classes: ClassDefinition::from_assignments([(GlyphId::new(4), 3)]),
        rules: vec![
            ContextRule::chained([1], 2, [], [3], [SubstitutionRecord::new(0, 7)]),
            ContextRule::chained([9], 2, [], [3], [SubstitutionRecord::new(0, 8)]),
        ],
    };

    let Some(ChainedContextSubstitution::ClassRules {
        coverage,
        backtrack_classes,
        input_classes,
        lookahead_classes,
        rules,
    }) = retain(&input, &closed([1, 3, 4]))
    else {
        panic!("one feasible chained class rule must remain");
    };

    assert_eq!(coverage.glyphs(), [GlyphId::new(3)]);
    assert_eq!(backtrack_classes.class_of(GlyphId::new(1)), 1);
    assert_eq!(input_classes.class_of(GlyphId::new(3)), 2);
    assert_eq!(lookahead_classes.class_of(GlyphId::new(4)), 3);
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].records(), [SubstitutionRecord::new(0, 7)]);
}

fn closed(raw_glyphs: impl IntoIterator<Item = u16>) -> GsubClosure {
    GsubClosure {
        glyphs: raw_glyphs
            .into_iter()
            .map(GlyphId::new)
            .collect::<BTreeSet<_>>(),
        lookups: BTreeSet::new(),
        passes: 0,
    }
}

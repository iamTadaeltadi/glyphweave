use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::apply;
use crate::{
    ChainedContextSubstitution, ClassDefinition, ContextRule, ContextSubstitution, Coverage,
    GsubSubtable, SubstitutionRecord,
};

#[test]
fn exact_and_class_contexts_reach_their_nested_lookups() {
    let exact = GsubSubtable::Context(ContextSubstitution::GlyphRules(vec![ContextRule::new(
        GlyphId::new(3),
        [GlyphId::new(4)],
        [SubstitutionRecord::new(0, 7)],
    )]));
    let classes = GsubSubtable::Context(ContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        classes: ClassDefinition::from_assignments([(GlyphId::new(3), 1), (GlyphId::new(4), 2)]),
        rules: vec![ContextRule::new(1, [2], [SubstitutionRecord::new(0, 8)])],
    });
    let chained = GsubSubtable::ChainedContext(ChainedContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        backtrack_classes: ClassDefinition::from_assignments([(GlyphId::new(1), 1)]),
        input_classes: ClassDefinition::from_assignments([(GlyphId::new(3), 2)]),
        lookahead_classes: ClassDefinition::from_assignments([(GlyphId::new(4), 3)]),
        rules: vec![ContextRule::chained(
            [1],
            2,
            [],
            [3],
            [SubstitutionRecord::new(0, 9)],
        )],
    });
    let glyphs = [GlyphId::new(1), GlyphId::new(3), GlyphId::new(4)]
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut lookups = BTreeSet::new();

    apply(&exact, &glyphs, &mut lookups);
    apply(&classes, &glyphs, &mut lookups);
    apply(&chained, &glyphs, &mut lookups);

    assert_eq!(lookups.into_iter().collect::<Vec<_>>(), [7, 8, 9]);
}

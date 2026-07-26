use glyphweave_sfnt::GlyphId;

use super::ChainedContextSubstitution;
use crate::{ClassDefinition, ContextRule, Coverage, SubstitutionRecord};

#[test]
fn emptiness_tracks_actionable_rules_in_every_chained_domain() {
    let empty_glyphs = ChainedContextSubstitution::<GlyphId>::GlyphRules(Vec::new());
    let glyphs = ChainedContextSubstitution::GlyphRules(vec![ContextRule::chained(
        [GlyphId::new(2)],
        GlyphId::new(3),
        [],
        [GlyphId::new(4)],
        [SubstitutionRecord::new(0, 1)],
    )]);
    let empty_classes = ChainedContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        backtrack_classes: ClassDefinition::default(),
        input_classes: ClassDefinition::default(),
        lookahead_classes: ClassDefinition::default(),
        rules: Vec::new(),
    };
    let empty_coverage = ChainedContextSubstitution::CoverageRule(ContextRule::new(
        Coverage::from_glyphs([GlyphId::new(3)]),
        [],
        [],
    ));

    assert!(empty_glyphs.is_empty());
    assert_eq!(empty_glyphs.len(), 0);
    assert!(!glyphs.is_empty());
    assert_eq!(glyphs.len(), 1);
    assert!(empty_classes.is_empty());
    assert_eq!(empty_classes.len(), 0);
    assert!(empty_coverage.is_empty());
    assert_eq!(empty_coverage.len(), 1);
}

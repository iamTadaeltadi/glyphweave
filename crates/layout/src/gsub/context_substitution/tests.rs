use glyphweave_sfnt::GlyphId;

use super::ContextSubstitution;
use crate::{ClassDefinition, ContextRule, Coverage, SubstitutionRecord};

#[test]
fn emptiness_tracks_actionable_rules_in_every_context_domain() {
    let empty_glyphs = ContextSubstitution::<GlyphId>::GlyphRules(Vec::new());
    let glyphs = ContextSubstitution::GlyphRules(vec![ContextRule::new(
        GlyphId::new(3),
        [],
        [SubstitutionRecord::new(0, 1)],
    )]);
    let empty_classes = ContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        classes: ClassDefinition::default(),
        rules: Vec::new(),
    };
    let empty_coverage = ContextSubstitution::CoverageRule(ContextRule::new(
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

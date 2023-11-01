use glyphweave_sfnt::{GlyphId, NewGlyphId};

use super::{chained, context, direct, ligature, reverse};
use crate::{
    AlternateSubstitution, ChainedContextSubstitution, ClassDefinition, ContextRule,
    ContextSubstitution, Coverage, LigatureRule, LigatureSubstitution, MultipleSubstitution,
    ReverseChainSingleSubstitution, SingleSubstitution, SubstitutionRecord,
};

#[test]
fn sequence_substitutions_preserve_output_order_and_deletion() {
    let multiple = MultipleSubstitution::from_mappings([
        (GlyphId::new(9), vec![GlyphId::new(12), GlyphId::new(4)]),
        (GlyphId::new(3), Vec::new()),
    ]);
    let alternate = AlternateSubstitution::from_alternates([(
        GlyphId::new(7),
        vec![GlyphId::new(10), GlyphId::new(8)],
    )]);
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 20));

    let multiple = direct::multiple(&multiple, &mut mapping).unwrap();
    let alternate = direct::alternate(&alternate, &mut mapping).unwrap();

    assert_eq!(multiple.replacement(NewGlyphId::new(23)), Some(&[][..]));
    assert_eq!(
        multiple.replacement(NewGlyphId::new(29)),
        Some(&[NewGlyphId::new(32), NewGlyphId::new(24)][..])
    );
    assert_eq!(
        alternate.alternates(NewGlyphId::new(27)),
        Some(&[NewGlyphId::new(30), NewGlyphId::new(28)][..])
    );
}

#[test]
fn ligature_and_reverse_context_order_survive_remapping() {
    let ligature = LigatureSubstitution::new([LigatureRule::new(
        GlyphId::new(2),
        GlyphId::new(3),
        [GlyphId::new(4)],
        GlyphId::new(9),
    )]);
    let reverse = ReverseChainSingleSubstitution::new(
        [Coverage::from_glyphs([GlyphId::new(8), GlyphId::new(1)])],
        SingleSubstitution::from_mappings([(GlyphId::new(3), GlyphId::new(7))]),
        [Coverage::from_glyphs([GlyphId::new(4)])],
    );
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 10));

    let ligature = ligature::substitution(&ligature, &mut mapping).unwrap();
    let reverse = reverse::substitution(&reverse, &mut mapping).unwrap();

    assert_eq!(
        ligature.rules()[0].components(),
        [
            NewGlyphId::new(12),
            NewGlyphId::new(13),
            NewGlyphId::new(14)
        ]
    );
    assert_eq!(ligature.rules()[0].replacement(), NewGlyphId::new(19));
    assert_eq!(
        reverse.backtrack()[0].glyphs(),
        [NewGlyphId::new(11), NewGlyphId::new(18)]
    );
    assert_eq!(
        reverse.substitutions().replacement(NewGlyphId::new(13)),
        Some(NewGlyphId::new(17))
    );
    assert_eq!(reverse.lookahead()[0].glyphs(), [NewGlyphId::new(14)]);
}

#[test]
fn every_contextual_predicate_domain_uses_output_glyphs() {
    let glyphs = ContextSubstitution::GlyphRules(vec![ContextRule::new(
        GlyphId::new(3),
        [GlyphId::new(4)],
        [SubstitutionRecord::new(1, 7)],
    )]);
    let classes = ContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        classes: ClassDefinition::from_assignments([(GlyphId::new(3), 1), (GlyphId::new(4), 2)]),
        rules: vec![ContextRule::new(1, [2], [SubstitutionRecord::new(0, 8)])],
    };
    let coverages = ContextSubstitution::CoverageRule(ContextRule::new(
        Coverage::from_glyphs([GlyphId::new(3), GlyphId::new(5)]),
        [Coverage::from_glyphs([GlyphId::new(4)])],
        [SubstitutionRecord::new(0, 9)],
    ));
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 20));

    let ContextSubstitution::GlyphRules(glyphs) =
        context::substitution(&glyphs, &mut mapping).unwrap()
    else {
        panic!("glyph rules must preserve their predicate domain");
    };
    let ContextSubstitution::ClassRules {
        coverage,
        classes,
        rules,
    } = context::substitution(&classes, &mut mapping).unwrap()
    else {
        panic!("class rules must preserve their predicate domain");
    };
    let ContextSubstitution::CoverageRule(coverages) =
        context::substitution(&coverages, &mut mapping).unwrap()
    else {
        panic!("Coverage rules must preserve their predicate domain");
    };

    assert_eq!(
        glyphs[0].input(),
        [NewGlyphId::new(23), NewGlyphId::new(24)]
    );
    assert_eq!(coverage.glyphs(), [NewGlyphId::new(23)]);
    assert_eq!(classes.class_of(NewGlyphId::new(24)), 2);
    assert_eq!(rules[0].input(), [1, 2]);
    assert_eq!(
        coverages.input()[0].glyphs(),
        [NewGlyphId::new(23), NewGlyphId::new(25)]
    );
    assert_eq!(coverages.records()[0], SubstitutionRecord::new(0, 9));
}

#[test]
fn every_chained_predicate_direction_uses_output_glyphs() {
    let glyphs = ChainedContextSubstitution::GlyphRules(vec![ContextRule::chained(
        [GlyphId::new(2)],
        GlyphId::new(3),
        [GlyphId::new(4)],
        [GlyphId::new(5)],
        [SubstitutionRecord::new(1, 7)],
    )]);
    let classes = ChainedContextSubstitution::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        backtrack_classes: ClassDefinition::from_assignments([(GlyphId::new(2), 1)]),
        input_classes: ClassDefinition::from_assignments([(GlyphId::new(3), 2)]),
        lookahead_classes: ClassDefinition::from_assignments([(GlyphId::new(4), 3)]),
        rules: vec![ContextRule::chained(
            [1],
            2,
            [],
            [3],
            [SubstitutionRecord::new(0, 8)],
        )],
    };
    let coverages = ChainedContextSubstitution::CoverageRule(ContextRule::chained(
        [Coverage::from_glyphs([GlyphId::new(2)])],
        Coverage::from_glyphs([GlyphId::new(3)]),
        [],
        [Coverage::from_glyphs([GlyphId::new(4)])],
        [SubstitutionRecord::new(0, 9)],
    ));
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 30));

    let ChainedContextSubstitution::GlyphRules(glyphs) =
        chained::substitution(&glyphs, &mut mapping).unwrap()
    else {
        panic!("glyph rules must preserve their chained domain");
    };
    let ChainedContextSubstitution::ClassRules {
        coverage,
        backtrack_classes,
        input_classes,
        lookahead_classes,
        rules,
    } = chained::substitution(&classes, &mut mapping).unwrap()
    else {
        panic!("class rules must preserve their chained domain");
    };
    let ChainedContextSubstitution::CoverageRule(coverages) =
        chained::substitution(&coverages, &mut mapping).unwrap()
    else {
        panic!("Coverage rules must preserve their chained domain");
    };

    assert_eq!(glyphs[0].backtrack(), [NewGlyphId::new(32)]);
    assert_eq!(
        glyphs[0].input(),
        [NewGlyphId::new(33), NewGlyphId::new(34)]
    );
    assert_eq!(glyphs[0].lookahead(), [NewGlyphId::new(35)]);
    assert_eq!(coverage.glyphs(), [NewGlyphId::new(33)]);
    assert_eq!(backtrack_classes.class_of(NewGlyphId::new(32)), 1);
    assert_eq!(input_classes.class_of(NewGlyphId::new(33)), 2);
    assert_eq!(lookahead_classes.class_of(NewGlyphId::new(34)), 3);
    assert_eq!(rules[0].lookahead(), [3]);
    assert_eq!(coverages.backtrack()[0].glyphs(), [NewGlyphId::new(32)]);
    assert_eq!(coverages.lookahead()[0].glyphs(), [NewGlyphId::new(34)]);
}

use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::ChainedContextPositioning;
use crate::{ClassDefinition, ContextRule, Coverage, SubstitutionRecord};

fn decode(bytes: &[u8]) -> ChainedContextPositioning {
    ChainedContextPositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap()
}

fn decode_cause(bytes: &[u8]) -> LayoutCause {
    let error =
        ChainedContextPositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();
    match error.cause() {
        Cause::Layout(cause) => cause,
        other => panic!("expected a layout cause, found {other:?}"),
    }
}

#[test]
fn emptiness_tracks_actionable_rules_in_every_chained_domain() {
    let empty_glyphs = ChainedContextPositioning::<GlyphId>::GlyphRules(Vec::new());
    let glyphs = ChainedContextPositioning::GlyphRules(vec![ContextRule::chained(
        [GlyphId::new(2)],
        GlyphId::new(3),
        [],
        [GlyphId::new(4)],
        [SubstitutionRecord::new(0, 1)],
    )]);
    let empty_classes = ChainedContextPositioning::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        backtrack_classes: ClassDefinition::default(),
        input_classes: ClassDefinition::default(),
        lookahead_classes: ClassDefinition::default(),
        rules: Vec::new(),
    };
    let empty_coverage = ChainedContextPositioning::CoverageRule(ContextRule::new(
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

#[test]
fn decodes_format_1_exact_glyph_chained_rule() {
    let bytes = [
        0x00, 0x01, 0x00, 0x26, 0x00, 0x01, 0x00, 0x08, // subtable header
        0x00, 0x01, 0x00, 0x04, // one ChainPosRule
        0x00, 0x02, 0x00, 0x02, 0x00, 0x01, // backtrack [2, 1]
        0x00, 0x03, 0x00, 0x04, 0x00, 0x05, // inputs [implicit 3, 4, 5]
        0x00, 0x01, 0x00, 0x09, // lookahead [9]
        0x00, 0x02, 0x00, 0x01, 0x00, 0x07, // first action
        0x00, 0x00, 0x00, 0x05, // second action
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // first-input Coverage {3}
    ];

    let ChainedContextPositioning::GlyphRules(rules) = decode(&bytes) else {
        panic!("format 1 must expand exact glyph rules");
    };
    assert_eq!(rules[0].backtrack(), [GlyphId::new(2), GlyphId::new(1)]);
    assert_eq!(
        rules[0].input(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5)]
    );
    assert_eq!(rules[0].lookahead(), [GlyphId::new(9)]);
    assert_eq!(
        rules[0].records(),
        [SubstitutionRecord::new(1, 7), SubstitutionRecord::new(0, 5)]
    );
}

#[test]
fn decodes_format_2_independent_class_domains() {
    let bytes = [
        0x00, 0x02, 0x00, 0x4a, 0x00, 0x2c, 0x00, 0x36, // format, Coverage, classes
        0x00, 0x42, 0x00, 0x03, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, // sets
        0x00, 0x01, 0x00, 0x04, // one ChainPosClassRule
        0x00, 0x02, 0x00, 0x02, 0x00, 0x01, // backtrack classes [2,1]
        0x00, 0x03, 0x00, 0x02, 0x00, 0x00, // input classes [implicit 1,2,0]
        0x00, 0x01, 0x00, 0x04, // lookahead class [4]
        0x00, 0x01, 0x00, 0x01, 0x00, 0x07, // action
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, // backtrack ClassDef
        0x00, 0x01, 0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, // input ClassDef
        0x00, 0x00, // input class zero
        0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00, 0x04, // lookahead ClassDef
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x05, // Coverage
    ];

    let ChainedContextPositioning::ClassRules {
        coverage,
        backtrack_classes,
        input_classes,
        lookahead_classes,
        rules,
    } = decode(&bytes)
    else {
        panic!("format 2 must retain independent class domains");
    };
    assert_eq!(coverage.glyphs(), [GlyphId::new(3), GlyphId::new(5)]);
    assert_eq!(backtrack_classes.class_of(GlyphId::new(2)), 2);
    assert_eq!(input_classes.class_of(GlyphId::new(5)), 0);
    assert_eq!(lookahead_classes.class_of(GlyphId::new(9)), 4);
    assert_eq!(rules[0].backtrack(), [2, 1]);
    assert_eq!(rules[0].input(), [1, 2, 0]);
    assert_eq!(rules[0].lookahead(), [4]);
    assert_eq!(rules[0].records(), [SubstitutionRecord::new(1, 7)]);
}

#[test]
fn decodes_format_3_three_direction_coverage_context() {
    let bytes = [
        0x00, 0x03, // format 3
        0x00, 0x02, 0x00, 0x1c, 0x00, 0x22, // two backtrack Coverages
        0x00, 0x02, 0x00, 0x28, 0x00, 0x2e, // two input Coverages
        0x00, 0x01, 0x00, 0x34, // one lookahead Coverage
        0x00, 0x02, 0x00, 0x01, 0x00, 0x07, // first action
        0x00, 0x00, 0x00, 0x05, // second action
        0x00, 0x01, 0x00, 0x01, 0x00, 0x02, // nearest backtrack {2}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x01, // farthest backtrack {1}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // first input {3}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x04, // second input {4}
        0x00, 0x01, 0x00, 0x01, 0x00, 0x09, // lookahead {9}
    ];

    let ChainedContextPositioning::CoverageRule(rule) = decode(&bytes) else {
        panic!("format 3 must retain Coverage predicates");
    };
    assert_eq!(rule.backtrack()[0].glyphs(), [GlyphId::new(2)]);
    assert_eq!(rule.backtrack()[1].glyphs(), [GlyphId::new(1)]);
    assert_eq!(rule.input()[0].glyphs(), [GlyphId::new(3)]);
    assert_eq!(rule.input()[1].glyphs(), [GlyphId::new(4)]);
    assert_eq!(rule.lookahead()[0].glyphs(), [GlyphId::new(9)]);
    assert_eq!(
        rule.records(),
        [SubstitutionRecord::new(1, 7), SubstitutionRecord::new(0, 5)]
    );
}

#[test]
fn rejects_unsupported_subtable_format() {
    let bytes = [0x00, 0x09];
    assert_eq!(
        decode_cause(&bytes),
        LayoutCause::UnsupportedGposChainedContextFormat { format: 9 }
    );
}

#[test]
fn rejects_empty_format_3_input_sequence() {
    let bytes = [
        0x00, 0x03, // format 3
        0x00, 0x00, // no backtrack Coverages
        0x00, 0x00, // no input Coverages
    ];
    assert_eq!(
        decode_cause(&bytes),
        LayoutCause::EmptyGposChainedContextInput
    );
}

#[test]
fn rejects_out_of_range_sequence_index() {
    let bytes = [
        0x00, 0x03, // format 3
        0x00, 0x00, // no backtrack Coverages
        0x00, 0x01, 0x00, 0x0e, // one input Coverage
        0x00, 0x00, // no lookahead Coverages
        0x00, 0x01, 0x00, 0x01, 0x00, 0x05, // action targeting absent position 1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // input Coverage {3}
    ];
    assert_eq!(
        decode_cause(&bytes),
        LayoutCause::InvalidGposChainedContextSequenceIndex {
            sequence_index: 1,
            input_count: 1,
        }
    );
}

#[test]
fn rejects_format_1_rule_set_count_mismatch() {
    let bytes = [
        0x00, 0x01, // format 1
        0x00, 0x0a, // Coverage offset
        0x00, 0x02, // two rule sets
        0x00, 0x00, 0x00, 0x00, // both null
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];
    assert_eq!(
        decode_cause(&bytes),
        LayoutCause::MismatchedGposChainedContextRuleSetCount { inputs: 1, sets: 2 }
    );
}

#[test]
fn layout_diagnostics_render_gpos_chained_context_causes() {
    assert_eq!(
        LayoutCause::UnsupportedGposChainedContextFormat { format: 4 }.to_string(),
        "unsupported GPOS ChainContextPos format 4"
    );
    assert_eq!(
        LayoutCause::MismatchedGposChainedContextRuleSetCount { inputs: 1, sets: 2 }.to_string(),
        "GPOS ChainContextPos covers 1 glyph(s) but declares 2 rule set(s)"
    );
    assert_eq!(
        LayoutCause::EmptyGposChainedContextInput.to_string(),
        "GPOS ChainContextPos rule has no input glyphs"
    );
    assert_eq!(
        LayoutCause::InvalidGposChainedContextSequenceIndex {
            sequence_index: 3,
            input_count: 2,
        }
        .to_string(),
        "GPOS ChainContextPos sequence index 3 is outside 2 input glyph(s)"
    );
}

#[test]
fn layout_targets_render_gpos_chained_context_names() {
    assert_eq!(
        LayoutTarget::GposChainedContextCoverage.to_string(),
        "GPOS ChainContextPos Coverage"
    );
    assert_eq!(
        LayoutTarget::GposChainedContextClassDefinition.to_string(),
        "GPOS ChainContextPos Class Definition"
    );
    assert_eq!(
        LayoutTarget::GposChainedContextRuleSet.to_string(),
        "GPOS ChainContextPos rule set"
    );
    assert_eq!(
        LayoutTarget::GposChainedContextRule.to_string(),
        "GPOS ChainContextPos rule"
    );
}

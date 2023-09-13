//! Integration coverage for ContextSubst decoding.

use glyphweave_layout::{ContextSubstitution, SubstitutionRecord};
use glyphweave_sfnt::{ByteSpan, GlyphId};

#[test]
fn decodes_format_3_coverage_predicates_and_action_order() {
    let bytes = [
        0x00, 0x03, 0x00, 0x02, 0x00, 0x02, // format, glyphCount, substCount
        0x00, 0x12, 0x00, 0x18, // Coverage offsets
        0x00, 0x01, 0x00, 0x09, // apply Lookup 9 to input position 1
        0x00, 0x00, 0x00, 0x04, // then Lookup 4 to input position 0
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // first Coverage {3}
        0x00, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x07, // second {5, 7}
    ];
    let substitution =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    let ContextSubstitution::CoverageRule(rule) = substitution else {
        panic!("format 3 must retain Coverage predicates");
    };
    assert_eq!(rule.input()[0].glyphs(), [GlyphId::new(3)]);
    assert_eq!(rule.input()[1].glyphs(), [GlyphId::new(5), GlyphId::new(7)]);
    assert_eq!(
        rule.records(),
        [SubstitutionRecord::new(1, 9), SubstitutionRecord::new(0, 4),]
    );
}

#[test]
fn decodes_format_1_rules_and_null_rule_set() {
    let bytes = [
        0x00, 0x01, 0x00, 0x2a, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x00, // header
        0x00, 0x02, 0x00, 0x06, 0x00, 0x16, // two SubRule offsets
        0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, // rule inputs
        0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x04, // rule actions
        0x00, 0x02, 0x00, 0x01, 0x00, 0x06, // second rule input
        0x00, 0x00, 0x00, 0x08, // second rule action
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // Coverage
    ];
    let substitution =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    let ContextSubstitution::GlyphRules(rules) = substitution else {
        panic!("format 1 must expand exact-glyph predicates");
    };
    assert_eq!(rules.len(), 2);
    assert_eq!(
        rules[0].input(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5),]
    );
    assert_eq!(
        rules[0].records(),
        [SubstitutionRecord::new(1, 9), SubstitutionRecord::new(0, 4),]
    );
    assert_eq!(rules[1].input(), [GlyphId::new(3), GlyphId::new(6)]);
}

#[test]
fn decodes_format_2_class_rules_and_implicit_class_zero() {
    let bytes = [
        0x00, 0x02, 0x00, 0x2a, 0x00, 0x1e, 0x00, 0x03, // header
        0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, // only class 1 has a SubClassSet
        0x00, 0x01, 0x00, 0x04, // one SubClassRule
        0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, // classes 1,2,0
        0x00, 0x02, 0x00, 0x09, // apply Lookup 9 at input position 2
        0x00, 0x01, 0x00, 0x03, 0x00, 0x03, // ClassDef format 1
        0x00, 0x01, 0x00, 0x02, 0x00, 0x00, // glyph 3->1, 4->2, 5->0
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x05, // Coverage {3,5}
    ];
    let substitution =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    let ContextSubstitution::ClassRules {
        coverage,
        classes,
        rules,
    } = substitution
    else {
        panic!("format 2 must retain its class domain");
    };
    assert_eq!(coverage.glyphs(), [GlyphId::new(3), GlyphId::new(5)]);
    assert_eq!(classes.class_of(GlyphId::new(3)), 1);
    assert_eq!(classes.class_of(GlyphId::new(5)), 0);
    assert_eq!(rules[0].input(), [1, 2, 0]);
    assert_eq!(rules[0].records(), [SubstitutionRecord::new(2, 9)]);
}

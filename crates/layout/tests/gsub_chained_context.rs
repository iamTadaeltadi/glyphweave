//! Integration coverage for `ChainContextSubst` decoding.

use glyphweave_layout::{ChainedContextSubstitution, SubstitutionRecord};
use glyphweave_sfnt::{ByteSpan, GlyphId};

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
    let substitution =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap();

    let ChainedContextSubstitution::CoverageRule(rule) = substitution else {
        panic!("format 3 must retain Coverage predicates");
    };
    assert_eq!(rule.backtrack()[0].glyphs(), [GlyphId::new(2)]);
    assert_eq!(rule.backtrack()[1].glyphs(), [GlyphId::new(1)]);
    assert_eq!(rule.input()[0].glyphs(), [GlyphId::new(3)]);
    assert_eq!(rule.input()[1].glyphs(), [GlyphId::new(4)]);
    assert_eq!(rule.lookahead()[0].glyphs(), [GlyphId::new(9)]);
    assert_eq!(
        rule.records(),
        [SubstitutionRecord::new(1, 7), SubstitutionRecord::new(0, 5),]
    );
}

#[test]
fn decodes_format_1_exact_glyph_chained_rule() {
    let bytes = [
        0x00, 0x01, 0x00, 0x26, 0x00, 0x01, 0x00, 0x08, // subtable header
        0x00, 0x01, 0x00, 0x04, // one ChainSubRule
        0x00, 0x02, 0x00, 0x02, 0x00, 0x01, // backtrack [2, 1]
        0x00, 0x03, 0x00, 0x04, 0x00, 0x05, // inputs [implicit 3, 4, 5]
        0x00, 0x01, 0x00, 0x09, // lookahead [9]
        0x00, 0x02, 0x00, 0x01, 0x00, 0x07, // first action
        0x00, 0x00, 0x00, 0x05, // second action
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // first-input Coverage {3}
    ];
    let substitution =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap();

    let ChainedContextSubstitution::GlyphRules(rules) = substitution else {
        panic!("format 1 must expand exact glyph rules");
    };
    assert_eq!(rules[0].backtrack(), [GlyphId::new(2), GlyphId::new(1)]);
    assert_eq!(
        rules[0].input(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5),]
    );
    assert_eq!(rules[0].lookahead(), [GlyphId::new(9)]);
    assert_eq!(
        rules[0].records(),
        [SubstitutionRecord::new(1, 7), SubstitutionRecord::new(0, 5),]
    );
}

#[test]
fn decodes_format_2_independent_class_domains() {
    let bytes = [
        0x00, 0x02, 0x00, 0x4a, 0x00, 0x2c, 0x00, 0x36, // format, Coverage, classes
        0x00, 0x42, 0x00, 0x03, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, // sets
        0x00, 0x01, 0x00, 0x04, // one ChainSubClassRule
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
    let substitution =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap();

    let ChainedContextSubstitution::ClassRules {
        coverage,
        backtrack_classes,
        input_classes,
        lookahead_classes,
        rules,
    } = substitution
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
}

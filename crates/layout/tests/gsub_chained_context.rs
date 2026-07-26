//! Integration coverage for ChainContextSubst decoding.

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

use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

use super::{ContextPositioning, ContextPositioningRecord, ContextPositioningRule};
use crate::{ClassDefinition, Coverage};

fn decode(bytes: &[u8]) -> ContextPositioning {
    ContextPositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap()
}

fn decode_error(bytes: &[u8]) -> Cause {
    ContextPositioning::decode(bytes, ByteSpan::between(0, bytes.len()).unwrap())
        .unwrap_err()
        .cause()
}

#[test]
fn emptiness_tracks_actionable_rules_in_every_context_domain() {
    let empty_glyphs = ContextPositioning::<GlyphId>::GlyphRules(Vec::new());
    let glyphs = ContextPositioning::GlyphRules(vec![ContextPositioningRule::new(
        GlyphId::new(3),
        [],
        [ContextPositioningRecord::new(0, 1)],
    )]);
    let empty_classes = ContextPositioning::ClassRules {
        coverage: Coverage::from_glyphs([GlyphId::new(3)]),
        classes: ClassDefinition::default(),
        rules: Vec::new(),
    };
    let empty_coverage = ContextPositioning::CoverageRule(ContextPositioningRule::new(
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
fn decodes_format_3_coverage_predicates_and_action_order() {
    let bytes = [
        0x00, 0x03, 0x00, 0x02, 0x00, 0x02, // format, glyphCount, posCount
        0x00, 0x12, 0x00, 0x18, // Coverage offsets
        0x00, 0x01, 0x00, 0x09, // apply Lookup 9 to input position 1
        0x00, 0x00, 0x00, 0x04, // then Lookup 4 to input position 0
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // first Coverage {3}
        0x00, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x07, // second {5, 7}
    ];

    let ContextPositioning::CoverageRule(rule) = decode(&bytes) else {
        panic!("format 3 must retain Coverage predicates");
    };
    assert_eq!(rule.input()[0].glyphs(), [GlyphId::new(3)]);
    assert_eq!(rule.input()[1].glyphs(), [GlyphId::new(5), GlyphId::new(7)]);
    assert_eq!(
        rule.records(),
        [
            ContextPositioningRecord::new(1, 9),
            ContextPositioningRecord::new(0, 4),
        ]
    );
}

#[test]
fn decodes_format_1_rules_and_null_rule_set() {
    let bytes = [
        0x00, 0x01, 0x00, 0x2a, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x00, // header
        0x00, 0x02, 0x00, 0x06, 0x00, 0x16, // two PosRule offsets
        0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, // rule inputs
        0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x04, // rule actions
        0x00, 0x02, 0x00, 0x01, 0x00, 0x06, // second rule input
        0x00, 0x00, 0x00, 0x08, // second rule action
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // Coverage
    ];

    let ContextPositioning::GlyphRules(rules) = decode(&bytes) else {
        panic!("format 1 must expand exact-glyph predicates");
    };
    assert_eq!(rules.len(), 2);
    assert_eq!(
        rules[0].input(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5)]
    );
    assert_eq!(
        rules[0].records(),
        [
            ContextPositioningRecord::new(1, 9),
            ContextPositioningRecord::new(0, 4),
        ]
    );
    assert_eq!(rules[1].input(), [GlyphId::new(3), GlyphId::new(6)]);
}

#[test]
fn decodes_format_2_class_rules_and_implicit_class_zero() {
    let bytes = [
        0x00, 0x02, 0x00, 0x2a, 0x00, 0x1e, 0x00, 0x03, // header
        0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, // only class 1 has a PosClassSet
        0x00, 0x01, 0x00, 0x04, // one PosClassRule
        0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, // classes 1,2,0
        0x00, 0x02, 0x00, 0x09, // apply Lookup 9 at input position 2
        0x00, 0x01, 0x00, 0x03, 0x00, 0x03, // ClassDef format 1
        0x00, 0x01, 0x00, 0x02, 0x00, 0x00, // glyph 3->1, 4->2, 5->0
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x05, // Coverage {3,5}
    ];

    let ContextPositioning::ClassRules {
        coverage,
        classes,
        rules,
    } = decode(&bytes)
    else {
        panic!("format 2 must retain its class domain");
    };
    assert_eq!(coverage.glyphs(), [GlyphId::new(3), GlyphId::new(5)]);
    assert_eq!(classes.class_of(GlyphId::new(3)), 1);
    assert_eq!(classes.class_of(GlyphId::new(5)), 0);
    assert_eq!(rules[0].input(), [1, 2, 0]);
    assert_eq!(rules[0].records(), [ContextPositioningRecord::new(2, 9)]);
}

#[test]
fn every_strict_format_prefix_is_rejected() {
    let format_1 = [
        0x00, 0x01, 0x00, 0x2a, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x02, 0x00, 0x06, 0x00,
        0x16, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00,
        0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x03, 0x00, 0x07,
    ];
    let format_2 = [
        0x00, 0x02, 0x00, 0x2a, 0x00, 0x1e, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x04, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x09,
        0x00, 0x01, 0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x03, 0x00, 0x05,
    ];
    let format_3 = [
        0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x04,
    ];
    for valid in [
        format_1.as_slice(),
        format_2.as_slice(),
        format_3.as_slice(),
    ] {
        for end in 0..valid.len() {
            let span = ByteSpan::between(0, end).unwrap();
            assert!(ContextPositioning::decode(&valid[..end], span).is_err());
        }
    }
}

#[test]
fn rejects_unknown_format() {
    assert_eq!(
        decode_error(&[0x00, 0x04]),
        Cause::Layout(LayoutCause::UnsupportedGposContextFormat { format: 4 })
    );
}

#[test]
fn format_three_rejects_empty_input() {
    assert_eq!(
        decode_error(&[0x00, 0x03, 0x00, 0x00]),
        Cause::Layout(LayoutCause::EmptyGposContextInput)
    );
}

#[test]
fn action_must_target_an_input_position() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x03,
    ];
    assert_eq!(
        decode_error(&bytes),
        Cause::Layout(LayoutCause::InvalidGposContextSequenceIndex {
            sequence_index: 1,
            input_count: 1,
        })
    );
}

#[test]
fn format_one_sets_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    assert_eq!(
        decode_error(&bytes),
        Cause::Layout(LayoutCause::MismatchedGposContextCount {
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn format_three_coverage_cannot_target_header() {
    let bytes = [0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06];
    assert_eq!(
        decode_error(&bytes),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::GposContextCoverage,
            offset: 6,
            minimum: 8,
            container_length: 8,
        })
    );
}

#[test]
fn class_definition_cannot_target_header() {
    let bytes = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    assert_eq!(
        decode_error(&bytes),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::GposContextClassDefinition,
            offset: 6,
            minimum: 8,
            container_length: 14,
        })
    );
}

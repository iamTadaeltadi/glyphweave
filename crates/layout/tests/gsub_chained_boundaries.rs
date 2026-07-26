//! Malformed-input boundaries for chained contextual GSUB decoding.

use glyphweave_layout::ChainedContextSubstitution;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTarget};

#[test]
fn every_strict_chained_format_one_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x26, 0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00,
        0x02, 0x00, 0x01, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x01, 0x00, 0x09, 0x00, 0x02,
        0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x05, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ChainedContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_chained_format_two_prefix_is_rejected() {
    let valid = [
        0x00, 0x02, 0x00, 0x4a, 0x00, 0x2c, 0x00, 0x36, 0x00, 0x42, 0x00, 0x03, 0x00, 0x00, 0x00,
        0x12, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x03,
        0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x00, 0x07, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x03, 0x00, 0x03,
        0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x05,
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ChainedContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_chained_format_three_prefix_is_rejected() {
    let valid = [
        0x00, 0x03, 0x00, 0x02, 0x00, 0x1c, 0x00, 0x22, 0x00, 0x02, 0x00, 0x28, 0x00, 0x2e, 0x00,
        0x01, 0x00, 0x34, 0x00, 0x02, 0x00, 0x01, 0x00, 0x07, 0x00, 0x00, 0x00, 0x05, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x01, 0x00, 0x01, 0x00, 0x09,
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ChainedContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn chained_context_rejects_unknown_format() {
    let bytes = [0x00, 0x04];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 6,
            format: 4,
        })
    );
}

#[test]
fn chained_format_three_rejects_empty_input() {
    let bytes = [0x00, 0x03, 0x00, 0x00, 0x00, 0x00];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::EmptyContextInput { lookup_type: 6 })
    );
}

#[test]
fn chained_action_must_target_an_input_position() {
    let bytes = [
        0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidSubstitutionSequenceIndex {
            lookup_type: 6,
            sequence_index: 1,
            input_count: 1,
        })
    );
}

#[test]
fn chained_input_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 10,
            minimum: 12,
            container_length: 12,
        })
    );
}

#[test]
fn chained_backtrack_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 12,
            minimum: 14,
            container_length: 14,
        })
    );
}

#[test]
fn chained_lookahead_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 12,
            minimum: 14,
            container_length: 20,
        })
    );
}

#[test]
fn chained_format_one_sets_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 6,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn chained_glyph_rule_set_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x06, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::ChainedContextRuleSet,
            offset: 6,
            minimum: 8,
            container_length: 14,
        })
    );
}

#[test]
fn chained_glyph_rule_cannot_target_set_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::ChainedContextRule,
            offset: 2,
            minimum: 4,
            container_length: 10,
        })
    );
}

#[test]
fn chained_glyph_rule_rejects_empty_input() {
    let bytes = [
        0x00, 0x01, 0x00, 0x10, 0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::EmptyContextInput { lookup_type: 6 })
    );
}

#[test]
fn chained_glyph_rule_bounds_action_positions() {
    let bytes = [
        0x00, 0x01, 0x00, 0x18, 0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ChainedContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidSubstitutionSequenceIndex {
            lookup_type: 6,
            sequence_index: 1,
            input_count: 1,
        })
    );
}

//! Malformed-input boundaries for contextual GSUB decoding.

use glyphweave_layout::ContextSubstitution;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTarget};

#[test]
fn every_strict_context_format_one_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x2a, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x02, 0x00, 0x06, 0x00,
        0x16, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, 0x00, 0x01, 0x00, 0x09, 0x00, 0x00,
        0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x03, 0x00, 0x07,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_context_format_two_prefix_is_rejected() {
    let valid = [
        0x00, 0x02, 0x00, 0x2a, 0x00, 0x1e, 0x00, 0x03, 0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x04, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x09,
        0x00, 0x01, 0x00, 0x03, 0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x02, 0x00, 0x03, 0x00, 0x05,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_context_format_three_prefix_is_rejected() {
    let valid = [
        0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x14, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x04,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn contextual_substitution_rejects_unknown_format() {
    let bytes = [0x00, 0x04];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 5,
            format: 4,
        })
    );
}

#[test]
fn context_format_three_rejects_empty_input() {
    let bytes = [0x00, 0x03, 0x00, 0x00];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, 4).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::EmptyContextInput { lookup_type: 5 })
    );
}

#[test]
fn context_action_must_target_an_input_position() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidSubstitutionSequenceIndex {
            lookup_type: 5,
            sequence_index: 1,
            input_count: 1,
        })
    );
}

#[test]
fn context_format_three_coverage_cannot_target_header() {
    let bytes = [0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 6,
            minimum: 8,
            container_length: 8,
        })
    );
}

#[test]
fn context_format_one_sets_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 5,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn context_class_definition_cannot_target_header() {
    let bytes = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x06, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::ContextClassDefinition,
            offset: 6,
            minimum: 8,
            container_length: 14,
        })
    );
}

//! Malformed-input boundaries for reverse-chained GSUB decoding.

use glyphweave_layout::ReverseChainSingleSubstitution;
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

#[test]
fn every_strict_reverse_chain_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x26, 0x00, 0x02, 0x00, 0x14, 0x00, 0x1a, 0x00, 0x01, 0x00, 0x20, 0x00,
        0x02, 0x00, 0x1e, 0x00, 0x28, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
        0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00,
        0x04,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ReverseChainSingleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn reverse_chain_rejects_unknown_format() {
    let bytes = [0x00, 0x02];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 8,
            format: 2,
        })
    );
}

#[test]
fn reverse_outputs_must_match_input_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 8,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn reverse_input_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
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
fn reverse_backtrack_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert!(matches!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 12,
            minimum: 14,
            ..
        })
    ));
}

#[test]
fn reverse_lookahead_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert!(matches!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 12,
            minimum: 14,
            ..
        })
    ));
}

#[test]
fn reverse_output_array_cannot_cross_subtable_end() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04,
    ];
    let span = ByteSpan::between(0, bytes.len()).unwrap();

    assert!(ReverseChainSingleSubstitution::decode(&bytes, span).is_err());
}

#[test]
fn reverse_chain_accepts_empty_context_sequences() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x03,
    ];
    let decoded =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap();

    assert!(decoded.backtrack().is_empty());
    assert!(decoded.lookahead().is_empty());
    assert_eq!(
        decoded.substitutions().replacement(GlyphId::new(3)),
        Some(GlyphId::new(4))
    );
}

//! Malformed-input boundaries for semantic GSUB decoding.

use glyphweave_layout::SingleSubstitution;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTarget};

#[test]
fn every_strict_single_delta_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x06, 0x00, 0x01, // SingleSubst delta +1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(SingleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_single_explicit_prefix_is_rejected() {
    let valid = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, // explicit output
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(SingleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn single_coverage_cannot_target_its_header() {
    let bytes = [0x00, 0x01, 0x00, 0x04, 0x00, 0x01];
    let diagnostic =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 4,
            minimum: 6,
            container_length: 6,
        })
    );
}

#[test]
fn single_explicit_outputs_must_match_coverage_length() {
    let bytes = [
        0x00, 0x02, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 1,
            inputs: 1,
            records: 2,
        })
    );
}

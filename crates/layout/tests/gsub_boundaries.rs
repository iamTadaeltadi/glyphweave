//! Malformed-input boundaries for semantic GSUB decoding.

use glyphweave_layout::{MultipleSubstitution, SingleSubstitution};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

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
        0x00, 0x02, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
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

#[test]
fn single_delta_offsets_remain_relative_inside_absolute_spans() {
    let bytes = [
        0xaa, 0xbb, 0x00, 0x01, 0x00, 0x06, 0x00, 0x02, // delta +2
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let decoded =
        SingleSubstitution::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(decoded.replacement(GlyphId::new(3)), Some(GlyphId::new(5)));
}

#[test]
fn single_explicit_outputs_preserve_maximum_glyph_id() {
    let bytes = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0xff, 0xff, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let decoded =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        decoded.replacement(GlyphId::new(3)),
        Some(GlyphId::new(u16::MAX))
    );
}

#[test]
fn every_strict_multiple_substitution_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x08,
        0x00, 0x02, 0x00, 0x05, 0x00, 0x06,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(MultipleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn multiple_substitution_rejects_unknown_format() {
    let bytes = [0x00, 0x02];
    let diagnostic =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 2,
            format: 2,
        })
    );
}

#[test]
fn multiple_sequence_count_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic = MultipleSubstitution::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 2,
            inputs: 1,
            records: 2,
        })
    );
}

//! Semantic decoding tests for OpenType Feature tables.

use glyphweave_layout::Feature;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause, LayoutTarget};

#[test]
fn decodes_lookup_set_and_absolute_parameters_position() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x0a, // featureParamsOffset
        0x00, 0x03, // lookupIndexCount
        0x00, 0x09, 0x00, 0x02, 0x00, 0x09, // lookupListIndices
        0xfe, // opaque Feature Parameters
    ];
    let feature = Feature::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(feature.parameters_position(), Some(12));
    assert_eq!(feature.lookup_indices(), &[2, 9]);
}

#[test]
fn decodes_null_parameters_offset() {
    let bytes = [0x00, 0x00, 0x00, 0x01, 0x00, 0x07];
    let feature = Feature::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(feature.parameters_position(), None);
    assert_eq!(feature.lookup_indices(), &[7]);
}

#[test]
fn rejects_parameters_target_inside_lookup_array() {
    let bytes = [0x00, 0x04, 0x00, 0x01, 0x00, 0x07, 0xfe];
    let diagnostic =
        Feature::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(0, 2));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::FeatureParameters,
            offset: 4,
            minimum: 6,
            container_length: 7,
        })
    );
}

#[test]
fn rejects_parameters_target_at_container_end() {
    let bytes = [0x00, 0x04, 0x00, 0x00];
    let diagnostic =
        Feature::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::FeatureParameters,
            offset: 4,
            minimum: 4,
            container_length: 4,
        })
    );
}

#[test]
fn every_strict_feature_prefix_is_rejected() {
    let complete = [
        0x00, 0x00, 0x00, 0x02, // null parameters, lookupIndexCount
        0x00, 0x01, 0x00, 0x03,
    ];
    for end in 0..complete.len() {
        let span = ByteSpan::between(0, end).unwrap();
        let diagnostic = Feature::decode(&complete, span).unwrap_err();
        assert!(
            matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }),
            "prefix {end} produced {}",
            diagnostic.cause()
        );
    }
}

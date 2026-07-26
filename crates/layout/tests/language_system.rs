//! Semantic decoding tests for OpenType Language System tables.

use glyphweave_layout::LanguageSystem;
use glyphweave_sfnt::{ByteSpan, Cause, LayoutCause};

#[test]
fn decodes_required_and_canonical_optional_features() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x00, // lookupOrderOffset
        0x00, 0x07, // requiredFeatureIndex
        0x00, 0x04, // featureIndexCount
        0x00, 0x09, 0x00, 0x02, 0x00, 0x09, 0x00, 0x04,
    ];
    let span = ByteSpan::between(2, bytes.len()).unwrap();
    let language = LanguageSystem::decode(&bytes, span).unwrap();

    assert_eq!(language.required_feature(), Some(7));
    assert_eq!(language.feature_indices(), &[2, 4, 9]);
}

#[test]
fn decodes_absent_required_feature() {
    let bytes = [0x00, 0x00, 0xff, 0xff, 0x00, 0x00];
    let span = ByteSpan::between(0, bytes.len()).unwrap();
    let language = LanguageSystem::decode(&bytes, span).unwrap();

    assert_eq!(language.required_feature(), None);
    assert!(language.feature_indices().is_empty());
}

#[test]
fn rejects_nonzero_reserved_lookup_order_offset() {
    let bytes = [0xaa, 0x00, 0x04, 0xff, 0xff, 0x00, 0x00];
    let span = ByteSpan::between(1, bytes.len()).unwrap();
    let diagnostic = LanguageSystem::decode(&bytes, span).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(1, 3));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::ReservedLookupOrderOffset { offset: 4 })
    );
}

#[test]
fn every_strict_language_system_prefix_is_rejected() {
    let complete = [
        0x00, 0x00, 0xff, 0xff, 0x00, 0x02, // header
        0x00, 0x01, 0x00, 0x03,
    ];
    for end in 0..complete.len() {
        let span = ByteSpan::between(0, end).unwrap();
        let diagnostic = LanguageSystem::decode(&complete, span).unwrap_err();
        assert!(
            matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }),
            "prefix {end} produced {}",
            diagnostic.cause()
        );
    }
}

//! Semantic decoding tests for OpenType Language System tables.

use glyphweave_layout::LanguageSystem;
use glyphweave_sfnt::ByteSpan;

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

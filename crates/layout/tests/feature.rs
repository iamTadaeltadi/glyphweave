//! Semantic decoding tests for OpenType Feature tables.

use glyphweave_layout::Feature;
use glyphweave_sfnt::ByteSpan;

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

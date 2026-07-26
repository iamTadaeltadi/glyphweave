//! Semantic decoding tests for OpenType Feature Lists.

use glyphweave_layout::FeatureList;
use glyphweave_sfnt::{ByteSpan, Tag};

const KERN: Tag = Tag::new(*b"kern");
const LIGA: Tag = Tag::new(*b"liga");

#[test]
fn decodes_stable_duplicate_feature_records() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x03, // featureCount
        b'k', b'e', b'r', b'n', 0x00, 0x14, // kern
        b'l', b'i', b'g', b'a', 0x00, 0x18, // liga implementation one
        b'l', b'i', b'g', b'a', 0x00, 0x1e, // liga implementation two
        0x00, 0x00, 0x00, 0x00, // kern Feature
        0x00, 0x00, 0x00, 0x01, 0x00, 0x07, // first liga Feature
        0x00, 0x04, 0x00, 0x00, 0xfe, // second liga Feature + parameters
    ];
    let list = FeatureList::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(list.tag(0), Some(KERN));
    assert_eq!(list.feature(1).unwrap().lookup_indices(), &[7]);
    assert_eq!(list.feature(2).unwrap().parameters_position(), Some(36));
    assert_eq!(
        list.features_with_tag(LIGA)
            .map(|(index, _)| index)
            .collect::<Vec<_>>(),
        [1, 2]
    );
}

#[test]
fn decodes_empty_feature_list() {
    let bytes = [0x00, 0x00];
    let list = FeatureList::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();
    assert!(list.is_empty());
}

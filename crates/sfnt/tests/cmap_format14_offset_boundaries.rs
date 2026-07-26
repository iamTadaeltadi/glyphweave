//! Nested-offset coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, Cmap14TableKind, CmapCause, CmapFormat14};
use support::encode_selector;

#[test]
fn default_offset_cannot_target_selector_record_bytes() {
    let mut bytes = encode_selector(0xfe0f, &[], &[]);
    bytes[13..17].copy_from_slice(&12_u32.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("target precedes record-array end");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14Offset {
            selector: 0xfe0f,
            kind: Cmap14TableKind::Default,
            offset: 12,
            records_end: 21,
            declared_length: 29,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(13, 4));
}

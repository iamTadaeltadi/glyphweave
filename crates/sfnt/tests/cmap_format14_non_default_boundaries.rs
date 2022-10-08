//! Non-default UVS mapping coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, CmapCause, CmapFormat14};
use support::{NonDefaultMapping, encode_selector};

#[test]
fn non_default_base_values_must_be_strictly_ordered() {
    let bytes = encode_selector(
        0xe0100,
        &[],
        &[
            NonDefaultMapping {
                base: 0x82a6,
                glyph: 7,
            },
            NonDefaultMapping {
                base: 0x82a6,
                glyph: 9,
            },
        ],
    );

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("base value is duplicated");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14NonDefaultMapping {
            selector: 0xe0100,
            index: 1,
            codepoint: 0x82a6,
            previous: Some(0x82a6),
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(34, 5));
}

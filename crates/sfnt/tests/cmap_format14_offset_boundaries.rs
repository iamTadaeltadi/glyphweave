//! Nested-offset coverage for `cmap` format 14 decoding.

#[path = "support/cmap_format14.rs"]
pub mod support;

use glyphweave_sfnt::{
    ByteSpan, Cause, Cmap14TableKind, CmapCause, CmapFormat14, VariationMapping, VariationSelector,
};
use support::{DefaultRange, NonDefaultMapping, encode_selector, push_u16, push_u24, push_u32};

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

#[test]
fn non_default_offset_must_locate_a_complete_count() {
    let mut bytes = encode_selector(0xfe0f, &[], &[]);
    bytes[17..21].copy_from_slice(&26_u32.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("count crosses declared end");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::InvalidCmap14Offset {
            selector: 0xfe0f,
            kind: Cmap14TableKind::NonDefault,
            offset: 26,
            records_end: 21,
            declared_length: 29,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(17, 4));
}

#[test]
fn default_range_count_cannot_exceed_target_capacity() {
    let mut bytes = encode_selector(0xfe0f, &[], &[]);
    bytes[21..25].copy_from_slice(&2_u32.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("two ranges do not fit");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::Cmap14EntryCountOutOfBounds {
            selector: 0xfe0f,
            kind: Cmap14TableKind::Default,
            count: 2,
            maximum: 1,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(21, 4));
}

#[test]
fn non_default_mapping_count_cannot_exceed_target_capacity() {
    let mut bytes = encode_selector(0xfe0f, &[], &[]);
    bytes[25..29].copy_from_slice(&1_u32.to_be_bytes());

    let diagnostic = CmapFormat14::decode(&bytes, 0).expect_err("one mapping does not fit");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap(CmapCause::Cmap14EntryCountOutOfBounds {
            selector: 0xfe0f,
            kind: Cmap14TableKind::NonDefault,
            count: 1,
            maximum: 0,
        })
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(25, 4));
}

#[test]
fn unreferenced_gap_before_target_tables_is_permitted() {
    let mut bytes = encode_selector(
        0xfe0f,
        &[DefaultRange {
            start: 0x2764,
            additional: 0,
        }],
        &[NonDefaultMapping {
            base: 0x82a6,
            glyph: 19,
        }],
    );
    bytes.splice(21..21, [0xaa; 3]);
    bytes[2..6].copy_from_slice(&41_u32.to_be_bytes());
    bytes[13..17].copy_from_slice(&24_u32.to_be_bytes());
    bytes[17..21].copy_from_slice(&32_u32.to_be_bytes());

    let decoded = CmapFormat14::decode(&bytes, 0).expect("offset layout can contain gaps");
    let selector = VariationSelector::new('\u{fe0f}').unwrap();
    assert_eq!(
        decoded
            .variation_selector_map()
            .mapping('\u{2764}', selector),
        Some(VariationMapping::Default)
    );
}

#[test]
fn selector_records_may_share_an_exact_default_table() {
    let mut bytes = Vec::new();
    push_u16(&mut bytes, 14);
    push_u32(&mut bytes, 40);
    push_u32(&mut bytes, 2);
    for selector in [0xfe0f, 0xe0100] {
        push_u24(&mut bytes, selector);
        push_u32(&mut bytes, 32);
        push_u32(&mut bytes, 0);
    }
    push_u32(&mut bytes, 1);
    push_u24(&mut bytes, 0x4e00);
    bytes.push(0);

    let decoded = CmapFormat14::decode(&bytes, 0).expect("exact target sharing is valid");
    for raw_selector in ['\u{fe0f}', '\u{e0100}'] {
        let selector = VariationSelector::new(raw_selector).unwrap();
        assert_eq!(
            decoded
                .variation_selector_map()
                .mapping('\u{4e00}', selector),
            Some(VariationMapping::Default)
        );
    }
    assert_eq!(decoded.variation_selector_map().default_range_count(), 2);
}

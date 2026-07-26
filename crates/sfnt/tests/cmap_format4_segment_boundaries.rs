//! Segment and range-boundary coverage for `cmap` format 4 decoding.

#[path = "support/cmap_format4.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, GlyphId};
use support::{decode_words, direct_delta_words, range_offset_words};

#[test]
fn segment_start_cannot_exceed_its_end() {
    let mut words = direct_delta_words();
    words[10] = 0x0044;

    let diagnostic = decode_words(&words).expect_err("first segment is inverted");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSegment {
            index: 0,
            start: 0x0044,
            end: 0x0043,
            previous_end: None,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(14, 8));
}

#[test]
fn segment_ranges_cannot_overlap_their_predecessor() {
    let words = [
        4, 40, 0, 6, 4, 1, 2, // header
        0x0043, 0x0045, 0xffff, // endCode
        0,      // reservedPad
        0x0041, 0x0043, 0xffff, // startCode
        0, 0, 1, // idDelta
        0, 0, 0, // idRangeOffset
    ];

    let diagnostic = decode_words(&words).expect_err("second segment overlaps first");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSegment {
            index: 1,
            start: 0x0043,
            end: 0x0045,
            previous_end: Some(0x0043),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 10));
}

#[test]
fn final_segment_must_be_the_single_code_unit_sentinel() {
    let mut words = direct_delta_words();
    words[11] = 0xfffe;

    let diagnostic = decode_words(&words).expect_err("sentinel starts below 0xffff");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSentinelRange {
            start: 0xfffe,
            end: 0xffff,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 8));
}

#[test]
fn sentinel_must_resolve_to_notdef() {
    let mut words = direct_delta_words();
    words[13] = 2;

    let diagnostic = decode_words(&words).expect_err("sentinel maps to glyph one");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapSentinel {
            start: 0xffff,
            end: 0xffff,
            glyph: GlyphId::new(1),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(26, 2));
}

#[test]
fn nonzero_surrogate_mapping_is_rejected() {
    let mut words = direct_delta_words();
    words[7] = 0xd800;
    words[10] = 0xd800;
    words[12] = 1;

    let diagnostic = decode_words(&words).expect_err("surrogate resolves nonzero");
    assert_eq!(
        diagnostic.cause(),
        Cause::CmapSurrogateMapping {
            codepoint: 0xd800,
            glyph: GlyphId::new(0xd801),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(24, 2));
}

#[test]
fn notdef_surrogate_mapping_is_canonicalized_to_absence() {
    let mut words = direct_delta_words();
    words[7] = 0xd800;
    words[10] = 0xd800;
    words[12] = 0x2800;

    let decoded = decode_words(&words).expect("zero surrogate mapping is absent");
    assert!(decoded.character_map().is_empty());
}

#[test]
fn range_offset_must_be_even() {
    let mut words = range_offset_words();
    words[14] = 3;

    let diagnostic = decode_words(&words).expect_err("odd idRangeOffset is invalid");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapRangeOffset {
            segment: 0,
            codepoint: 0x0041,
            target: 31,
            glyph_array: ByteSpan::sized(32, 6).unwrap(),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(28, 2));
}

#[test]
fn range_offset_cannot_reinterpret_segment_arrays_as_glyphs() {
    let mut words = range_offset_words();
    words[14] = 2;

    let diagnostic = decode_words(&words).expect_err("target precedes glyphIdArray");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapRangeOffset {
            segment: 0,
            codepoint: 0x0041,
            target: 30,
            glyph_array: ByteSpan::sized(32, 6).unwrap(),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(28, 2));
}

#[test]
fn every_range_target_must_fit_inside_the_declared_subtable() {
    let mut words = range_offset_words();
    words[14] = 6;

    let diagnostic = decode_words(&words).expect_err("last glyph word is out of bounds");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmapRangeOffset {
            segment: 0,
            codepoint: 0x0043,
            target: 38,
            glyph_array: ByteSpan::sized(32, 6).unwrap(),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(28, 2));
}

use super::Cause;

#[test]
fn cmap_format_failure_names_expected_and_found_formats() {
    assert_eq!(
        Cause::UnsupportedCmapFormat {
            expected: 4,
            found: 12,
        }
        .to_string(),
        "expected cmap format 4 but found format 12"
    );
}

#[test]
fn cmap_length_failure_names_all_bounds() {
    let cause = Cause::InvalidCmapLength {
        declared: 20,
        minimum: 24,
        available: 22,
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 length 20 needs at least 24 byte(s) with 22 available"
    );
}

#[test]
fn cmap_odd_length_failure_names_declared_value() {
    assert_eq!(
        Cause::OddCmapLength { declared: 25 }.to_string(),
        "cmap format 4 length 25 is not even"
    );
}

#[test]
fn cmap_segment_count_failure_names_encoded_value() {
    assert_eq!(
        Cause::InvalidCmapSegmentCount { encoded: 3 }.to_string(),
        "cmap format 4 segCountX2 3 is not positive and even"
    );
}

#[test]
fn cmap_search_failure_compares_derived_values() {
    let cause = Cause::InvalidCmapSearchParameters {
        expected: [4, 1, 2],
        actual: [2, 0, 4],
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 search parameters [2, 0, 4] do not match derived values [4, 1, 2]"
    );
}

#[test]
fn cmap_reserved_pad_failure_names_found_word() {
    assert_eq!(
        Cause::InvalidCmapReservedPad { found: 1 }.to_string(),
        "cmap format 4 reservedPad must be zero but is 1"
    );
}

#[test]
fn cmap_segment_failure_names_range_and_predecessor() {
    let cause = Cause::InvalidCmapSegment {
        index: 2,
        start: 0x0030,
        end: 0x0020,
        previous_end: Some(0x001f),
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 segment 2 range 0x0030..=0x0020 is invalid after Some(31)"
    );
}

#[test]
fn cmap_sentinel_failure_names_range_and_glyph() {
    let cause = Cause::InvalidCmapSentinel {
        start: 0xffff,
        end: 0xffff,
        glyph: crate::GlyphId::new(7),
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 sentinel 0xffff..=0xffff resolves to glyph 7"
    );
}

#[test]
fn cmap_sentinel_range_failure_names_final_range() {
    let cause = Cause::InvalidCmapSentinelRange {
        start: 0xfffe,
        end: 0xffff,
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 final segment 0xfffe..=0xffff is not the 0xffff sentinel"
    );
}

#[test]
fn cmap_range_offset_failure_names_target_and_legal_span() {
    let cause = Cause::InvalidCmapRangeOffset {
        segment: 1,
        codepoint: 0x0041,
        target: 88,
        glyph_array: crate::ByteSpan::sized(100, 20).expect("test span is representable"),
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 segment 1 code 0x0041 targets byte 88 outside glyph array 100..120"
    );
}

#[test]
fn cmap_surrogate_failure_names_codepoint_and_glyph() {
    let cause = Cause::CmapSurrogateMapping {
        codepoint: 0xd800,
        glyph: crate::GlyphId::new(5),
    };
    assert_eq!(
        cause.to_string(),
        "cmap format 4 surrogate 0xd800 resolves to glyph 5"
    );
}

use super::Cause;

#[test]
fn cmap_format_failure_names_expected_and_found_formats() {
    assert_eq!(
        Cause::UnsupportedCmapFormat { found: 12 }.to_string(),
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

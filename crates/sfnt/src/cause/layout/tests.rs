use super::LayoutCause;
use crate::GlyphId;

#[test]
fn unsupported_coverage_format_names_the_selector() {
    let cause = LayoutCause::UnsupportedCoverageFormat { format: 9 };
    assert_eq!(cause.to_string(), "unsupported Coverage format 9");
}

#[test]
fn unsupported_class_definition_format_names_the_selector() {
    let cause = LayoutCause::UnsupportedClassDefinitionFormat { format: 7 };
    assert_eq!(cause.to_string(), "unsupported Class Definition format 7");
}

#[test]
fn coverage_ordering_cause_names_both_glyphs() {
    let cause = LayoutCause::NonIncreasingCoverageGlyph {
        previous: GlyphId::new(9),
        current: GlyphId::new(7),
    };
    assert_eq!(
        cause.to_string(),
        "Coverage glyph 7 does not strictly follow 9"
    );
}

#[test]
fn invalid_coverage_range_names_its_bounds() {
    let cause = LayoutCause::InvalidCoverageRange {
        start: GlyphId::new(12),
        end: GlyphId::new(10),
    };
    assert_eq!(
        cause.to_string(),
        "Coverage range 12..=10 has reversed bounds"
    );
}

#[test]
fn discontinuous_coverage_index_names_expected_and_actual() {
    let cause = LayoutCause::DiscontinuousCoverageIndex {
        expected: 20,
        actual: 18,
    };
    assert_eq!(
        cause.to_string(),
        "Coverage range index 18 does not continue at 20"
    );
}

#[test]
fn overlapping_coverage_range_names_the_boundary() {
    let cause = LayoutCause::OverlappingCoverageRange {
        previous_end: GlyphId::new(15),
        start: GlyphId::new(15),
    };
    assert_eq!(
        cause.to_string(),
        "Coverage range beginning at 15 overlaps preceding end 15"
    );
}

#[test]
fn class_range_overflow_names_start_and_count() {
    let cause = LayoutCause::ClassDefinitionGlyphRangeOverflow {
        start: GlyphId::new(65_534),
        count: 3,
    };
    assert_eq!(
        cause.to_string(),
        "Class Definition range beginning at 65534 with 3 value(s) exceeds glyph ID 65535"
    );
}

#[test]
fn invalid_class_range_names_its_bounds() {
    let cause = LayoutCause::InvalidClassDefinitionRange {
        start: GlyphId::new(8),
        end: GlyphId::new(7),
    };
    assert_eq!(
        cause.to_string(),
        "Class Definition range 8..=7 has reversed bounds"
    );
}

#[test]
fn overlapping_class_range_names_the_boundary() {
    let cause = LayoutCause::OverlappingClassDefinitionRange {
        previous_end: GlyphId::new(20),
        start: GlyphId::new(19),
    };
    assert_eq!(
        cause.to_string(),
        "Class Definition range beginning at 19 overlaps preceding end 20"
    );
}

use super::{LayoutCause, LayoutListKind, LayoutTarget};
use crate::{GlyphId, Tag};

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

#[test]
fn layout_list_kinds_have_stable_names() {
    assert_eq!(LayoutListKind::Script.to_string(), "script");
    assert_eq!(LayoutListKind::Language.to_string(), "language");
    assert_eq!(LayoutListKind::Feature.to_string(), "feature");
}

#[test]
fn layout_targets_have_stable_names() {
    assert_eq!(LayoutTarget::Script.to_string(), "Script");
    assert_eq!(
        LayoutTarget::DefaultLanguageSystem.to_string(),
        "default Language System"
    );
    assert_eq!(LayoutTarget::LanguageSystem.to_string(), "Language System");
    assert_eq!(LayoutTarget::Feature.to_string(), "Feature");
    assert_eq!(
        LayoutTarget::FeatureParameters.to_string(),
        "Feature Parameters"
    );
}

#[test]
fn invalid_layout_tag_names_its_list() {
    let cause = LayoutCause::InvalidLayoutTag {
        list: LayoutListKind::Script,
        tag: Tag::new([b'a', 0, b'c', b'd']),
    };
    assert_eq!(cause.to_string(), "invalid script tag Tag(a\\x00cd)");
}

#[test]
fn unsorted_layout_tags_name_the_adjacent_pair() {
    let cause = LayoutCause::UnsortedLayoutTags {
        list: LayoutListKind::Feature,
        previous: Tag::new(*b"liga"),
        current: Tag::new(*b"kern"),
    };
    assert_eq!(
        cause.to_string(),
        "feature tag kern follows larger tag liga"
    );
}

#[test]
fn duplicate_layout_tag_names_its_list() {
    let cause = LayoutCause::DuplicateLayoutTag {
        list: LayoutListKind::Language,
        tag: Tag::new(*b"ENG "),
    };
    assert_eq!(cause.to_string(), "duplicate language tag ENG ");
}

#[test]
fn invalid_layout_offset_names_its_legal_interval() {
    let cause = LayoutCause::InvalidLayoutOffset {
        target: LayoutTarget::LanguageSystem,
        offset: 6,
        minimum: 10,
        container_length: 42,
    };
    assert_eq!(
        cause.to_string(),
        "Language System offset 6 is outside legal range 10..42"
    );
}

#[test]
fn reserved_lookup_order_offset_names_the_value() {
    let cause = LayoutCause::ReservedLookupOrderOffset { offset: 4 };
    assert_eq!(
        cause.to_string(),
        "reserved lookup-order offset must be zero, not 4"
    );
}

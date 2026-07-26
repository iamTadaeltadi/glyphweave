use super::{LayoutCause, LayoutListKind, LayoutTableKind, LayoutTarget};
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
fn layout_table_kinds_use_standard_tags() {
    assert_eq!(LayoutTableKind::Substitution.to_string(), "GSUB");
    assert_eq!(LayoutTableKind::Positioning.to_string(), "GPOS");
}

#[test]
fn layout_table_kinds_define_lookup_boundaries() {
    assert_eq!(LayoutTableKind::Substitution.maximum_lookup_type(), 8);
    assert_eq!(LayoutTableKind::Substitution.extension_lookup_type(), 7);
    assert_eq!(LayoutTableKind::Positioning.maximum_lookup_type(), 9);
    assert_eq!(LayoutTableKind::Positioning.extension_lookup_type(), 9);
}

#[test]
fn layout_targets_have_stable_names() {
    assert_eq!(LayoutTarget::Lookup.to_string(), "Lookup");
    assert_eq!(LayoutTarget::LookupSubtable.to_string(), "Lookup subtable");
    assert_eq!(
        LayoutTarget::ExtensionSubtable.to_string(),
        "extension subtable"
    );
    assert_eq!(
        LayoutTarget::SubstitutionCoverage.to_string(),
        "substitution Coverage"
    );
    assert_eq!(
        LayoutTarget::SubstitutionSequence.to_string(),
        "substitution Sequence"
    );
    assert_eq!(LayoutTarget::AlternateSet.to_string(), "AlternateSet");
    assert_eq!(LayoutTarget::LigatureSet.to_string(), "LigatureSet");
    assert_eq!(LayoutTarget::Ligature.to_string(), "Ligature");
    assert_eq!(
        LayoutTarget::ContextClassDefinition.to_string(),
        "context Class Definition"
    );
    assert_eq!(LayoutTarget::ContextRuleSet.to_string(), "ContextRuleSet");
    assert_eq!(LayoutTarget::ContextRule.to_string(), "ContextRule");
    assert_eq!(
        LayoutTarget::ChainedContextRuleSet.to_string(),
        "ChainContextRuleSet"
    );
    assert_eq!(
        LayoutTarget::ChainedContextRule.to_string(),
        "ChainContextRule"
    );
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

#[test]
fn invalid_lookup_type_names_its_table() {
    let cause = LayoutCause::InvalidLookupType {
        table: LayoutTableKind::Substitution,
        lookup_type: 9,
    };
    assert_eq!(cause.to_string(), "invalid GSUB lookup type 9");
}

#[test]
fn unexpected_layout_family_names_both_tables() {
    let cause = LayoutCause::UnexpectedLayoutTableKind {
        expected: LayoutTableKind::Substitution,
        actual: LayoutTableKind::Positioning,
    };
    assert_eq!(cause.to_string(), "expected GSUB Lookup, found GPOS Lookup");
}

#[test]
fn empty_lookup_names_its_type() {
    let cause = LayoutCause::EmptyLookupSubtables {
        table: LayoutTableKind::Positioning,
        lookup_type: 3,
    };
    assert_eq!(cause.to_string(), "GPOS lookup type 3 has no subtables");
}

#[test]
fn reserved_lookup_flags_preserve_the_raw_word() {
    let cause = LayoutCause::ReservedLookupFlags { flags: 0x00a2 };
    assert_eq!(cause.to_string(), "lookup flags 0x00a2 set reserved bits");
}

#[test]
fn unsupported_extension_format_names_its_table() {
    let cause = LayoutCause::UnsupportedExtensionFormat {
        table: LayoutTableKind::Positioning,
        format: 2,
    };
    assert_eq!(cause.to_string(), "unsupported GPOS extension format 2");
}

#[test]
fn recursive_extension_names_the_transport_type() {
    let cause = LayoutCause::RecursiveExtensionLookupType {
        table: LayoutTableKind::Substitution,
        lookup_type: 7,
    };
    assert_eq!(
        cause.to_string(),
        "GSUB extension cannot target lookup type 7"
    );
}

#[test]
fn mismatched_extensions_name_both_real_types() {
    let cause = LayoutCause::MismatchedExtensionLookupTypes {
        table: LayoutTableKind::Positioning,
        expected: 4,
        actual: 6,
    };
    assert_eq!(
        cause.to_string(),
        "GPOS extension lookup type 6 differs from preceding type 4"
    );
}

#[test]
fn unsupported_gsub_format_names_type_and_selector() {
    let cause = LayoutCause::UnsupportedGsubFormat {
        lookup_type: 4,
        format: 3,
    };
    assert_eq!(cause.to_string(), "unsupported GSUB lookup type 4 format 3");
}

#[test]
fn mismatched_gsub_count_names_both_domains() {
    let cause = LayoutCause::MismatchedGsubMappingCount {
        lookup_type: 2,
        inputs: 3,
        records: 2,
    };
    assert_eq!(
        cause.to_string(),
        "GSUB lookup type 2 has 3 input glyph(s) but 2 mapping record(s)"
    );
}

#[test]
fn invalid_ligature_count_names_the_minimum() {
    let cause = LayoutCause::InvalidLigatureComponentCount { count: 1 };
    assert_eq!(
        cause.to_string(),
        "GSUB Ligature declares 1 component(s), fewer than the required two"
    );
}

#[test]
fn empty_context_names_its_lookup_type() {
    let cause = LayoutCause::EmptyContextInput { lookup_type: 6 };
    assert_eq!(
        cause.to_string(),
        "GSUB lookup type 6 context has no input glyphs"
    );
}

#[test]
fn invalid_context_sequence_index_names_its_bound() {
    let cause = LayoutCause::InvalidSubstitutionSequenceIndex {
        lookup_type: 5,
        sequence_index: 3,
        input_count: 3,
    };
    assert_eq!(
        cause.to_string(),
        "GSUB lookup type 5 sequence index 3 is outside 3 input glyph(s)"
    );
}

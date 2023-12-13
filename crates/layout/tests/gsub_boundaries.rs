//! Malformed-input boundaries for semantic GSUB decoding.

use glyphweave_layout::{
    AlternateSubstitution, ContextSubstitution, LigatureSubstitution, MultipleSubstitution,
    ReverseChainSingleSubstitution, SingleSubstitution,
};
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause, LayoutTarget};

#[test]
fn every_strict_single_delta_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x06, 0x00, 0x01, // SingleSubst delta +1
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(SingleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_single_explicit_prefix_is_rejected() {
    let valid = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, // explicit output
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03, // Coverage {3}
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(SingleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn single_coverage_cannot_target_its_header() {
    let bytes = [0x00, 0x01, 0x00, 0x04, 0x00, 0x01];
    let diagnostic =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 4,
            minimum: 6,
            container_length: 6,
        })
    );
}

#[test]
fn single_explicit_outputs_must_match_coverage_length() {
    let bytes = [
        0x00, 0x02, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    let diagnostic =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 1,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn single_delta_offsets_remain_relative_inside_absolute_spans() {
    let bytes = [
        0xaa, 0xbb, 0x00, 0x01, 0x00, 0x06, 0x00, 0x02, // delta +2
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let decoded =
        SingleSubstitution::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(decoded.replacement(GlyphId::new(3)), Some(GlyphId::new(5)));
}

#[test]
fn single_explicit_outputs_preserve_maximum_glyph_id() {
    let bytes = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0xff, 0xff, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let decoded =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        decoded.replacement(GlyphId::new(3)),
        Some(GlyphId::new(u16::MAX))
    );
}

#[test]
fn every_strict_multiple_substitution_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x08, 0x00, 0x02, 0x00, 0x05, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];

    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(MultipleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn multiple_substitution_rejects_unknown_format() {
    let bytes = [0x00, 0x02];
    let diagnostic =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 2,
            format: 2,
        })
    );
}

#[test]
fn multiple_sequence_count_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    let diagnostic =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 2,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn multiple_sequence_cannot_target_lookup_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x06, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionSequence,
            offset: 6,
            minimum: 8,
            container_length: 14,
        })
    );
}

#[test]
fn multiple_sequence_cannot_target_subtable_end() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        MultipleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionSequence,
            offset: 14,
            minimum: 8,
            container_length: 14,
        })
    );
}

#[test]
fn multiple_sequence_outputs_cannot_cross_subtable_end() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x02, 0x00, 0x04,
    ];
    let span = ByteSpan::between(0, bytes.len()).unwrap();

    assert!(MultipleSubstitution::decode(&bytes, span).is_err());
}

#[test]
fn every_strict_alternate_substitution_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x08, 0x00, 0x02, 0x00, 0x05, 0x00, 0x06, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(AlternateSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn alternate_substitution_rejects_unknown_format() {
    let bytes = [0x00, 0x02];
    let diagnostic =
        AlternateSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 3,
            format: 2,
        })
    );
}

#[test]
fn alternate_set_count_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    let diagnostic =
        AlternateSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 3,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn alternate_set_cannot_target_lookup_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x06, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        AlternateSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::AlternateSet,
            offset: 6,
            minimum: 8,
            container_length: 14,
        })
    );
}

#[test]
fn alternate_set_glyphs_cannot_cross_subtable_end() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x02, 0x00, 0x04,
    ];
    let span = ByteSpan::between(0, bytes.len()).unwrap();

    assert!(AlternateSubstitution::decode(&bytes, span).is_err());
}

#[test]
fn empty_alternate_set_remains_an_explicit_mapping() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x00,
    ];
    let decoded =
        AlternateSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(decoded.alternates(GlyphId::new(3)), Some(&[][..]));
}

#[test]
fn every_strict_ligature_substitution_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x12, 0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x04, 0x00, 0x07, 0x00,
        0x02, 0x00, 0x06, 0x00, 0x01, 0x00, 0x01, 0x00, 0x05,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(LigatureSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn ligature_substitution_rejects_unknown_format() {
    let bytes = [0x00, 0x02];
    let diagnostic =
        LigatureSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 4,
            format: 2,
        })
    );
}

#[test]
fn ligature_set_count_must_match_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03,
    ];
    let diagnostic =
        LigatureSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 4,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn ligature_set_cannot_target_lookup_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x06, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        LigatureSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::LigatureSet,
            offset: 6,
            minimum: 8,
            container_length: 14,
        })
    );
}

#[test]
fn ligature_record_cannot_target_set_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x03,
    ];
    let diagnostic =
        LigatureSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::Ligature,
            offset: 2,
            minimum: 4,
            container_length: 10,
        })
    );
}

#[test]
fn ligature_replacement_must_be_complete() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x01, 0x00, 0x04, 0xff,
    ];
    let span = ByteSpan::between(0, bytes.len()).unwrap();

    assert!(LigatureSubstitution::decode(&bytes, span).is_err());
}

#[test]
fn ligature_component_vector_cannot_cross_subtable_end() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x01, 0x00, 0x04, 0x00, 0x07, 0x00, 0x03, 0x00, 0x04,
    ];
    let span = ByteSpan::between(0, bytes.len()).unwrap();

    assert!(LigatureSubstitution::decode(&bytes, span).is_err());
}

#[test]
fn empty_ligature_set_decodes_without_rules() {
    let bytes = [
        0x00, 0x01, 0x00, 0x08, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00,
        0x00,
    ];
    let decoded =
        LigatureSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert!(decoded.is_empty());
}

#[test]
fn every_strict_reverse_chain_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x26, 0x00, 0x02, 0x00, 0x14, 0x00, 0x1a, 0x00, 0x01, 0x00, 0x20, 0x00,
        0x02, 0x00, 0x1e, 0x00, 0x28, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01,
        0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x09, 0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00,
        0x04,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ReverseChainSingleSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn reverse_chain_rejects_unknown_format() {
    let bytes = [0x00, 0x02];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 8,
            format: 2,
        })
    );
}

#[test]
fn reverse_outputs_must_match_input_coverage() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 8,
            inputs: 1,
            records: 2,
        })
    );
}

#[test]
fn reverse_input_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 10,
            minimum: 12,
            container_length: 12,
        })
    );
}

#[test]
fn reverse_backtrack_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert!(matches!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 12,
            minimum: 14,
            ..
        })
    ));
}

#[test]
fn reverse_lookahead_coverage_cannot_target_header() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x01, 0x00, 0x0c, 0x00, 0x01, 0x00, 0x04, 0x00,
        0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap_err();

    assert!(matches!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 12,
            minimum: 14,
            ..
        })
    ));
}

#[test]
fn reverse_output_array_cannot_cross_subtable_end() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x04,
    ];
    let span = ByteSpan::between(0, bytes.len()).unwrap();

    assert!(ReverseChainSingleSubstitution::decode(&bytes, span).is_err());
}

#[test]
fn reverse_chain_accepts_empty_context_sequences() {
    let bytes = [
        0x00, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x04, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x03,
    ];
    let decoded =
        ReverseChainSingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap())
            .unwrap();

    assert!(decoded.backtrack().is_empty());
    assert!(decoded.lookahead().is_empty());
    assert_eq!(
        decoded.substitutions().replacement(GlyphId::new(3)),
        Some(GlyphId::new(4))
    );
}

#[test]
fn every_strict_context_format_one_prefix_is_rejected() {
    let valid = [
        0x00, 0x01, 0x00, 0x2a, 0x00, 0x02, 0x00, 0x0a, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x06, 0x00, 0x16,
        0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x00, 0x05,
        0x00, 0x01, 0x00, 0x09, 0x00, 0x00, 0x00, 0x04,
        0x00, 0x02, 0x00, 0x01, 0x00, 0x06, 0x00, 0x00, 0x00, 0x08,
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_context_format_two_prefix_is_rejected() {
    let valid = [
        0x00, 0x02, 0x00, 0x2a, 0x00, 0x1e, 0x00, 0x03,
        0x00, 0x00, 0x00, 0x0e, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x04,
        0x00, 0x03, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x09,
        0x00, 0x01, 0x00, 0x03, 0x00, 0x03,
        0x00, 0x01, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x05,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn every_strict_context_format_three_prefix_is_rejected() {
    let valid = [
        0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00, 0x0e, 0x00, 0x14,
        0x00, 0x00, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x04,
    ];
    for end in 0..valid.len() {
        let span = ByteSpan::between(0, end).unwrap();
        assert!(ContextSubstitution::decode(&valid[..end], span).is_err());
    }
}

#[test]
fn contextual_substitution_rejects_unknown_format() {
    let bytes = [0x00, 0x04];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, 2).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 5,
            format: 4,
        })
    );
}

#[test]
fn context_format_three_rejects_empty_input() {
    let bytes = [0x00, 0x03, 0x00, 0x00];
    let diagnostic =
        ContextSubstitution::decode(&bytes, ByteSpan::between(0, 4).unwrap()).unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::EmptyContextInput { lookup_type: 5 })
    );
}

#[test]
fn context_action_must_target_an_input_position() {
    let bytes = [
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x0c,
        0x00, 0x01, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x01, 0x00, 0x03,
    ];
    let diagnostic = ContextSubstitution::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidSubstitutionSequenceIndex {
            lookup_type: 5,
            sequence_index: 1,
            input_count: 1,
        })
    );
}

#[test]
fn context_format_three_coverage_cannot_target_header() {
    let bytes = [0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00, 0x06];
    let diagnostic = ContextSubstitution::decode(
        &bytes,
        ByteSpan::between(0, bytes.len()).unwrap(),
    )
    .unwrap_err();

    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::InvalidLayoutOffset {
            target: LayoutTarget::SubstitutionCoverage,
            offset: 6,
            minimum: 8,
            container_length: 8,
        })
    );
}

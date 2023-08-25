//! Integration coverage for semantic SingleSubst decoding.

use glyphweave_layout::SingleSubstitution;
use glyphweave_sfnt::{ByteSpan, Cause, GlyphId, LayoutCause};

#[test]
fn decodes_wrapping_single_delta_substitution() {
    let bytes = [
        0xaa, 0xbb, // prefix
        0x00, 0x01, // format 1
        0x00, 0x06, // coverageOffset
        0xff, 0xfe, // deltaGlyphID = -2
        0x00, 0x01, // Coverage format 1
        0x00, 0x02, // glyphCount
        0x00, 0x01, 0xff, 0xff, // covered glyphs
    ];
    let substitution =
        SingleSubstitution::decode(&bytes, ByteSpan::between(2, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        substitution.replacement(GlyphId::new(1)),
        Some(GlyphId::new(65_535))
    );
    assert_eq!(
        substitution.replacement(GlyphId::new(65_535)),
        Some(GlyphId::new(65_533))
    );
}

#[test]
fn decodes_explicit_outputs_in_coverage_index_order() {
    let bytes = [
        0x00, 0x02, // format 2
        0x00, 0x0c, // coverageOffset
        0x00, 0x03, // glyphCount
        0x00, 0x1e, 0x00, 0x0a, 0x00, 0x28, // substitute glyphs 30, 10, 40
        0x00, 0x01, // Coverage format 1
        0x00, 0x03, // coverage glyphCount
        0x00, 0x03, 0x00, 0x07, 0x00, 0x09, // covered glyphs
    ];
    let substitution =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap();

    assert_eq!(
        substitution.mappings().collect::<Vec<_>>(),
        [
            (GlyphId::new(3), GlyphId::new(30)),
            (GlyphId::new(7), GlyphId::new(10)),
            (GlyphId::new(9), GlyphId::new(40)),
        ]
    );
}

#[test]
fn rejects_unknown_single_substitution_format() {
    let bytes = [0xaa, 0x00, 0x03];
    let diagnostic =
        SingleSubstitution::decode(&bytes, ByteSpan::between(1, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(1, 3));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::UnsupportedGsubFormat {
            lookup_type: 1,
            format: 3,
        })
    );
}

#[test]
fn rejects_single_output_count_different_from_coverage() {
    let bytes = [
        0x00, 0x02, 0x00, 0x08, 0x00, 0x01, 0x00, 0x14, // header and output
        0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x07, // two covered glyphs
    ];
    let diagnostic =
        SingleSubstitution::decode(&bytes, ByteSpan::between(0, bytes.len()).unwrap()).unwrap_err();

    assert_eq!(diagnostic.span(), ByteSpan::between(4, 6));
    assert_eq!(
        diagnostic.cause(),
        Cause::Layout(LayoutCause::MismatchedGsubMappingCount {
            lookup_type: 1,
            inputs: 2,
            records: 1,
        })
    );
}

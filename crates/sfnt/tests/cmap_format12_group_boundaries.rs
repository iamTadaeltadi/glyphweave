//! Group-boundary coverage for `cmap` format 12 decoding.

#[path = "support/cmap_format12.rs"]
pub mod support;

use glyphweave_sfnt::{ByteSpan, Cause, GlyphId};
use support::decode_groups;

#[test]
fn group_start_cannot_exceed_its_end() {
    let diagnostic =
        decode_groups(0, &[(0x10020, 0x10010, 1)]).expect_err("sequential group is inverted");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmap12Group {
            index: 0,
            start: 0x10020,
            end: 0x10010,
            previous_end: None,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 12));
}

#[test]
fn groups_cannot_overlap_their_predecessor() {
    let diagnostic = decode_groups(0, &[(0x0041, 0x0043, 1), (0x0043, 0x0044, 9)])
        .expect_err("second group overlaps first");
    assert_eq!(
        diagnostic.cause(),
        Cause::InvalidCmap12Group {
            index: 1,
            start: 0x0043,
            end: 0x0044,
            previous_end: Some(0x0043),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(28, 12));
}

#[test]
fn starting_glyph_must_fit_the_glyph_id_domain() {
    let diagnostic =
        decode_groups(0, &[(0x0041, 0x0041, 65_536)]).expect_err("starting glyph exceeds u16");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap12GlyphOutOfRange {
            group: 0,
            codepoint: 0x0041,
            glyph: 65_536,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 12));
}

#[test]
fn sequential_glyph_range_cannot_cross_the_glyph_id_boundary() {
    let diagnostic = decode_groups(0, &[(0x0041, 0x0043, 65_534)])
        .expect_err("third sequential glyph exceeds u16");
    assert_eq!(
        diagnostic.cause(),
        Cause::Cmap12GlyphOutOfRange {
            group: 0,
            codepoint: 0x0043,
            glyph: 65_536,
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 12));
}

#[test]
fn sequential_glyph_range_can_end_at_the_glyph_id_maximum() {
    let decoded =
        decode_groups(0, &[(0x0041, 0x0042, 65_534)]).expect("last mapping is exactly u16 max");
    assert_eq!(decoded.character_map().glyph_id('A'), GlyphId::new(65_534));
    assert_eq!(decoded.character_map().glyph_id('B'), GlyphId::new(65_535));
}

#[test]
fn nonzero_surrogate_mapping_is_rejected() {
    let diagnostic =
        decode_groups(0, &[(0xd800, 0xd800, 7)]).expect_err("surrogate is not a Unicode scalar");
    assert_eq!(
        diagnostic.cause(),
        Cause::CmapNonScalarMapping {
            format: 12,
            codepoint: 0xd800,
            glyph: GlyphId::new(7),
        }
    );
    assert_eq!(diagnostic.span(), ByteSpan::sized(16, 12));
}

#[test]
fn notdef_surrogate_mapping_is_canonicalized_to_absence() {
    let decoded =
        decode_groups(0, &[(0xd800, 0xd800, 0)]).expect("zero surrogate mapping is absent");
    assert!(decoded.character_map().is_empty());
}

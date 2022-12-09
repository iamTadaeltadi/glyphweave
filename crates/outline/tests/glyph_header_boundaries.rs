//! Boundary coverage for common TrueType Glyph Headers.

use glyphweave_outline::GlyphHeader;
use glyphweave_sfnt::{ByteSpan, Cause, OutlineCause};

#[test]
fn an_empty_loca_span_is_not_a_glyph_description() {
    let span = ByteSpan::empty_at(0);
    let diagnostic = GlyphHeader::decode(&[], span).expect_err("empty glyphs have no header");

    assert_eq!(diagnostic.span(), Some(span));
    assert_eq!(
        diagnostic.cause(),
        Cause::Outline(OutlineCause::EmptyGlyphDescription)
    );
}

#[test]
fn empty_glyph_span_still_must_be_inside_the_font() {
    let span = ByteSpan::empty_at(1);
    let diagnostic = GlyphHeader::decode(&[], span)
        .expect_err("an empty span after the font is not a checked loca result");

    assert_eq!(diagnostic.span(), Some(span));
    assert_eq!(
        diagnostic.cause(),
        Cause::SpanOutOfBounds { end: 1, length: 0 }
    );
}

#[test]
fn every_nonempty_strict_header_prefix_is_rejected() {
    let complete = [0; 10];

    for length in 1..complete.len() {
        let span = ByteSpan::sized(0, length).unwrap();
        let diagnostic = GlyphHeader::decode(&complete[..length], span)
            .expect_err("a Glyph Header occupies exactly ten bytes");

        assert!(matches!(diagnostic.cause(), Cause::UnexpectedEnd { .. }));
        assert_eq!(diagnostic.span().unwrap().end(), length);
    }
}

#[test]
fn glyph_span_must_be_inside_the_font_before_header_reads() {
    let bytes = [0; 11];
    let span = ByteSpan::sized(2, 10).unwrap();
    let diagnostic =
        GlyphHeader::decode(&bytes, span).expect_err("the declared glyph ends after the font");

    assert_eq!(diagnostic.span(), Some(span));
    assert_eq!(
        diagnostic.cause(),
        Cause::SpanOutOfBounds {
            end: 12,
            length: 11
        }
    );
}

#[test]
fn each_inverted_bounding_axis_is_rejected_at_the_bounds_fields() {
    let cases = [
        ([0, 0, 0, 9, 0, 0, 0, 8, 0, 0], (9, 0, 8, 0)),
        ([0, 0, 0, 0, 0, 9, 0, 0, 0, 8], (0, 9, 0, 8)),
    ];

    for (header, (minimum_x, minimum_y, maximum_x, maximum_y)) in cases {
        let mut bytes = vec![0xaa, 0xbb];
        bytes.extend_from_slice(&header);
        let glyph_span = ByteSpan::sized(2, 10).unwrap();
        let diagnostic = GlyphHeader::decode(&bytes, glyph_span)
            .expect_err("a lower-left bound cannot exceed its upper-right bound");

        assert_eq!(diagnostic.span(), ByteSpan::sized(4, 8));
        assert_eq!(
            diagnostic.cause(),
            Cause::Outline(OutlineCause::InvalidGlyphBounds {
                minimum_x,
                minimum_y,
                maximum_x,
                maximum_y,
            })
        );
    }
}

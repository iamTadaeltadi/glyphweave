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

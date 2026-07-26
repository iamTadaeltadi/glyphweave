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

//! Bounded decoding of the common TrueType Glyph Header.

use glyphweave_sfnt::{ByteSpan, Cause, Decoded, Diagnostic, OutlineCause, Reader, Stage};

use super::GlyphHeader;
use crate::GlyphBounds;

const CONTOUR_COUNT_LENGTH: usize = 2;
const BOUNDS_LENGTH: usize = 8;

pub(super) fn decode(bytes: &[u8], glyph_span: ByteSpan) -> Decoded<GlyphHeader> {
    glyph_span.slice(bytes)?;
    if glyph_span.is_empty() {
        return Err(Diagnostic::at(
            Stage::Decode,
            glyph_span,
            Cause::Outline(OutlineCause::EmptyGlyphDescription),
        ));
    }

    let mut reader = Reader::within(bytes, glyph_span)?;
    let number_of_contours = reader.i16()?;
    let minimum_x = reader.i16()?;
    let minimum_y = reader.i16()?;
    let maximum_x = reader.i16()?;
    let maximum_y = reader.i16()?;
    let bounds = GlyphBounds::new(minimum_x, minimum_y, maximum_x, maximum_y).ok_or_else(|| {
        Diagnostic::at(
            Stage::Decode,
            bounds_span(glyph_span),
            Cause::Outline(OutlineCause::InvalidGlyphBounds {
                minimum_x,
                minimum_y,
                maximum_x,
                maximum_y,
            }),
        )
    })?;

    Ok(GlyphHeader::new(number_of_contours, bounds))
}

fn bounds_span(glyph_span: ByteSpan) -> ByteSpan {
    glyph_span
        .start()
        .checked_add(CONTOUR_COUNT_LENGTH)
        .and_then(|start| ByteSpan::sized(start, BOUNDS_LENGTH))
        .unwrap_or(glyph_span)
}

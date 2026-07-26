//! Ordered eager decoding of `loca`-delimited glyph descriptions.

use glyphweave_sfnt::{Decoded, GlyphId};

use super::Glyf;
use crate::{CompositeGlyph, GlyphHeader, Loca, Outline, SimpleGlyph};

pub(super) fn decode(bytes: &[u8], loca: &Loca) -> Decoded<Glyf> {
    let table_span = loca.glyf_span();
    table_span.slice(bytes)?;

    let mut outlines = Vec::with_capacity(usize::from(loca.glyph_count()));
    for raw in 0..loca.glyph_count() {
        let glyph = GlyphId::new(raw);
        let glyph_span = loca
            .glyph_span(glyph)
            .expect("validated loca has one interval per in-range glyph");
        let outline = if glyph_span.is_empty() {
            Outline::Empty { span: glyph_span }
        } else if GlyphHeader::decode(bytes, glyph_span)?.is_composite() {
            Outline::Composite(CompositeGlyph::decode(bytes, glyph_span)?)
        } else {
            Outline::Simple(SimpleGlyph::decode(bytes, glyph_span)?)
        };
        outlines.push(outline);
    }

    Ok(Glyf {
        table_span,
        outlines,
    })
}

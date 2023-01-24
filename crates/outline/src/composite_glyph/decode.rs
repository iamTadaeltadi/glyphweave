//! Bounded orchestration of composite-glyph component records.

mod arguments;
mod component;
mod flags;
mod transform;

use glyphweave_sfnt::{ByteSpan, Cause, Decoded, Diagnostic, OutlineCause, Reader, Stage};

use super::CompositeGlyph;
use crate::GlyphHeader;

const HEADER_LENGTH: usize = 10;
const CONTOUR_COUNT_LENGTH: usize = 2;

pub(super) fn decode(bytes: &[u8], glyph_span: ByteSpan) -> Decoded<CompositeGlyph> {
    let header = GlyphHeader::decode(bytes, glyph_span)?;
    if !header.is_composite() {
        return Err(Diagnostic::at(
            Stage::Decode,
            field_span(glyph_span.start(), CONTOUR_COUNT_LENGTH),
            Cause::Outline(OutlineCause::SimpleGlyphForCompositeDecoder {
                number_of_contours: header.number_of_contours(),
            }),
        ));
    }

    let mut reader = Reader::within(bytes, glyph_span)?;
    reader.skip(HEADER_LENGTH)?;
    let mut components = Vec::new();
    let mut has_instructions = false;
    let mut overlapping_components = false;
    loop {
        let component_index = u32::try_from(components.len()).unwrap_or(u32::MAX);
        let decoded = component::decode(&mut reader, component_index)?;
        has_instructions |= decoded.flags.has_instructions();
        overlapping_components |= decoded.flags.has_overlap();
        let has_more = decoded.flags.has_more();
        components.push(decoded.component);
        if !has_more {
            break;
        }
    }

    let instructions = decode_instructions(&mut reader, has_instructions)?.to_vec();
    consume_alignment(&mut reader, glyph_span)?;
    Ok(CompositeGlyph {
        span: glyph_span,
        bounds: header.bounds(),
        components,
        instructions,
        overlapping_components,
    })
}

fn decode_instructions<'a>(reader: &mut Reader<'a>, present: bool) -> Decoded<&'a [u8]> {
    if !present {
        return Ok(&[]);
    }
    let length = usize::from(reader.u16()?);
    reader.array(length)
}

fn consume_alignment(reader: &mut Reader<'_>, glyph_span: ByteSpan) -> Decoded<()> {
    match reader.remaining() {
        0 => Ok(()),
        1 => {
            let offset = reader.offset();
            let value = reader.u8()?;
            if value == 0 {
                Ok(())
            } else {
                Err(Diagnostic::at(
                    Stage::Decode,
                    field_span(offset, 1),
                    Cause::Outline(OutlineCause::InvalidCompositeGlyphPadding { value }),
                ))
            }
        }
        length => Err(Diagnostic::at(
            Stage::Decode,
            ByteSpan::between(reader.offset(), glyph_span.end()).unwrap_or(glyph_span),
            Cause::Outline(OutlineCause::UnexpectedCompositeGlyphData { length }),
        )),
    }
}

fn field_span(start: usize, length: usize) -> ByteSpan {
    ByteSpan::sized(start, length).unwrap_or_else(|| ByteSpan::empty_at(start))
}

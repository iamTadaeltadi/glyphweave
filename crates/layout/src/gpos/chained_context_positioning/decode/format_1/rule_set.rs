//! Decoding for one Coverage-indexed `ChainPosRuleSet`.

use glyphweave_sfnt::{ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTarget, Reader};

use super::super::decode_records;
use crate::{ContextRule, layout_error, relative_target_span};

pub(super) fn decode(
    bytes: &[u8],
    span: ByteSpan,
    first: GlyphId,
) -> Decoded<Vec<ContextRule<GlyphId>>> {
    let mut reader = Reader::within(bytes, span)?;
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    let minimum = reader.offset() - span.start();
    offsets
        .into_iter()
        .map(|(position, offset)| {
            let target = relative_target_span(
                span,
                position,
                offset,
                minimum,
                LayoutTarget::GposChainedContextRule,
            )?;
            decode_rule(bytes, target, first)
        })
        .collect()
}

fn decode_rule(bytes: &[u8], span: ByteSpan, first: GlyphId) -> Decoded<ContextRule<GlyphId>> {
    let mut reader = Reader::within(bytes, span)?;
    let backtrack = read_glyphs(&mut reader)?;
    let count_position = reader.offset();
    let input_count = reader.u16()?;
    if input_count == 0 {
        return Err(layout_error(
            count_position,
            LayoutCause::EmptyGposChainedContextInput,
        ));
    }
    let mut input = Vec::with_capacity(usize::from(input_count - 1));
    for _ in 1..input_count {
        input.push(GlyphId::new(reader.u16()?));
    }
    let lookahead = read_glyphs(&mut reader)?;
    let record_count = reader.u16()?;
    let records = decode_records(&mut reader, record_count, input_count)?;
    Ok(ContextRule::chained(
        backtrack, first, input, lookahead, records,
    ))
}

fn read_glyphs(reader: &mut Reader<'_>) -> Decoded<Vec<GlyphId>> {
    let count = reader.u16()?;
    let mut glyphs = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        glyphs.push(GlyphId::new(reader.u16()?));
    }
    Ok(glyphs)
}

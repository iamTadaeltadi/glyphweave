//! Decoding for one class-indexed SubClassSet.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use crate::{ContextRule, gsub::decode::decode_records, layout_error, relative_target_span};

pub(super) fn decode(
    bytes: &[u8],
    span: ByteSpan,
    first_class: u16,
) -> Decoded<Vec<ContextRule<u16>>> {
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
                LayoutTarget::ContextRule,
            )?;
            decode_rule(bytes, target, first_class)
        })
        .collect()
}

fn decode_rule(bytes: &[u8], span: ByteSpan, first_class: u16) -> Decoded<ContextRule<u16>> {
    let mut reader = Reader::within(bytes, span)?;
    let count_position = reader.offset();
    let glyph_count = reader.u16()?;
    if glyph_count == 0 {
        return Err(layout_error(
            count_position,
            LayoutCause::EmptyContextInput { lookup_type: 5 },
        ));
    }
    let record_count = reader.u16()?;
    let mut remaining = Vec::with_capacity(usize::from(glyph_count - 1));
    for _ in 1..glyph_count {
        remaining.push(reader.u16()?);
    }
    let records = decode_records(&mut reader, record_count, glyph_count, 5)?;
    Ok(ContextRule::new(first_class, remaining, records))
}

//! Decoding for one class-indexed ChainSubClassSet.

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
                LayoutTarget::ChainedContextRule,
            )?;
            decode_rule(bytes, target, first_class)
        })
        .collect()
}

fn decode_rule(bytes: &[u8], span: ByteSpan, first_class: u16) -> Decoded<ContextRule<u16>> {
    let mut reader = Reader::within(bytes, span)?;
    let backtrack = read_classes(&mut reader)?;
    let count_position = reader.offset();
    let input_count = reader.u16()?;
    if input_count == 0 {
        return Err(layout_error(
            count_position,
            LayoutCause::EmptyContextInput { lookup_type: 6 },
        ));
    }
    let mut input = Vec::with_capacity(usize::from(input_count - 1));
    for _ in 1..input_count {
        input.push(reader.u16()?);
    }
    let lookahead = read_classes(&mut reader)?;
    let record_count = reader.u16()?;
    let records = decode_records(&mut reader, record_count, input_count, 6)?;
    Ok(ContextRule::chained(backtrack, first_class, input, lookahead, records))
}

fn read_classes(reader: &mut Reader<'_>) -> Decoded<Vec<u16>> {
    let count = reader.u16()?;
    let mut classes = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        classes.push(reader.u16()?);
    }
    Ok(classes)
}

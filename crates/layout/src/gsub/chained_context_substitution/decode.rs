//! Bounded ChainContextSubst format dispatch.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, Reader};

use super::ChainedContextSubstitution;
use crate::{
    ContextRule,
    gsub::decode::{decode_coverages, decode_records},
    layout_error,
};

mod format_1;

impl ChainedContextSubstitution {
    /// Decodes a ChainContextSubst subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid formats, offsets, Coverage, counts, or action indices.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => format_1::decode(bytes, span, &mut reader),
            3 => Self::decode_format_3(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGsubFormat {
                    lookup_type: 6,
                    format,
                },
            )),
        }
    }

    fn decode_format_3(bytes: &[u8], span: ByteSpan, reader: &mut Reader<'_>) -> Decoded<Self> {
        let backtrack_offsets = read_offsets(reader)?;
        let input_count_position = reader.offset();
        let input_offsets = read_offsets(reader)?;
        let input_count = u16::try_from(input_offsets.len()).unwrap_or(u16::MAX);
        if input_count == 0 {
            return Err(layout_error(
                input_count_position,
                LayoutCause::EmptyContextInput { lookup_type: 6 },
            ));
        }
        let lookahead_offsets = read_offsets(reader)?;
        let record_count = reader.u16()?;
        let records = decode_records(reader, record_count, input_count, 6)?;
        let minimum_position = reader.offset();
        let backtrack = decode_coverages(bytes, span, backtrack_offsets, minimum_position)?;
        let mut input = decode_coverages(bytes, span, input_offsets, minimum_position)?.into_iter();
        let first = input
            .next()
            .expect("nonzero input count decoded one Coverage");
        let lookahead = decode_coverages(bytes, span, lookahead_offsets, minimum_position)?;
        Ok(Self::CoverageRule(ContextRule::chained(
            backtrack, first, input, lookahead, records,
        )))
    }
}

fn read_offsets(reader: &mut Reader<'_>) -> Decoded<Vec<(usize, u16)>> {
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    Ok(offsets)
}

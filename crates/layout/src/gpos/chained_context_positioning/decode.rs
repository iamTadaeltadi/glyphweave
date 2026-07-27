//! Bounded `ChainContextPos` format dispatch and shared predicate decoding.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::ChainedContextPositioning;
use crate::{
    ClassDefinition, ContextRule, Coverage, SubstitutionRecord, layout_error, relative_target_span,
};

mod format_1;
mod format_2;

impl ChainedContextPositioning {
    /// Decodes a `ChainContextPos` subtable at the beginning of `span`.
    /// # Errors
    /// For truncation, invalid formats, offsets, Coverage, counts, or action indices.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        match reader.u16()? {
            1 => format_1::decode(bytes, span, &mut reader),
            2 => format_2::decode(bytes, span, &mut reader),
            3 => Self::decode_format_3(bytes, span, &mut reader),
            format => Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposChainedContextFormat { format },
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
                LayoutCause::EmptyGposChainedContextInput,
            ));
        }
        let lookahead_offsets = read_offsets(reader)?;
        let record_count = reader.u16()?;
        let records = decode_records(reader, record_count, input_count)?;
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

fn decode_coverage(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum_position: usize,
) -> Decoded<Coverage> {
    let target = relative_target_span(
        owner,
        position,
        offset,
        minimum_position - owner.start(),
        LayoutTarget::GposChainedContextCoverage,
    )?;
    Coverage::decode(&bytes[..owner.end()], target.start())
}

fn decode_coverages(
    bytes: &[u8],
    owner: ByteSpan,
    offsets: impl IntoIterator<Item = (usize, u16)>,
    minimum_position: usize,
) -> Decoded<Vec<Coverage>> {
    offsets
        .into_iter()
        .map(|(position, offset)| decode_coverage(bytes, owner, position, offset, minimum_position))
        .collect()
}

fn decode_classes(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum_position: usize,
) -> Decoded<ClassDefinition> {
    let target = relative_target_span(
        owner,
        position,
        offset,
        minimum_position - owner.start(),
        LayoutTarget::GposChainedContextClassDefinition,
    )?;
    ClassDefinition::decode(&bytes[..owner.end()], target.start())
}

fn decode_records(
    reader: &mut Reader<'_>,
    count: u16,
    input_count: u16,
) -> Decoded<Vec<SubstitutionRecord>> {
    let mut records = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        let sequence_index = reader.u16()?;
        let lookup_index = reader.u16()?;
        if sequence_index >= input_count {
            return Err(layout_error(
                position,
                LayoutCause::InvalidGposChainedContextSequenceIndex {
                    sequence_index,
                    input_count,
                },
            ));
        }
        records.push(SubstitutionRecord::new(sequence_index, lookup_index));
    }
    Ok(records)
}

//! Shared bounded targets used by the `ContextPos` format decoders.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use crate::{
    ClassDefinition, ContextPositioningRecord, Coverage, layout_error, relative_target_span,
};

pub(super) fn decode_coverage(
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
        LayoutTarget::GposContextCoverage,
    )?;
    Coverage::decode(&bytes[..owner.end()], target.start())
}

pub(super) fn decode_coverages(
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

pub(super) fn decode_records(
    reader: &mut Reader<'_>,
    count: u16,
    input_count: u16,
) -> Decoded<Vec<ContextPositioningRecord>> {
    let mut records = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let position = reader.offset();
        let sequence_index = reader.u16()?;
        let lookup_index = reader.u16()?;
        if sequence_index >= input_count {
            return Err(layout_error(
                position,
                LayoutCause::InvalidGposContextSequenceIndex {
                    sequence_index,
                    input_count,
                },
            ));
        }
        records.push(ContextPositioningRecord::new(sequence_index, lookup_index));
    }
    Ok(records)
}

pub(super) fn decode_classes(
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
        LayoutTarget::GposContextClassDefinition,
    )?;
    ClassDefinition::decode(&bytes[..owner.end()], target.start())
}

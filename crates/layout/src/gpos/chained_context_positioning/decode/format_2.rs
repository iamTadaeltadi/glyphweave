//! `ChainContextPos` format 2 class rule-set decoding.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutTarget, Reader};

use super::{ChainedContextPositioning, decode_classes, decode_coverage};
use crate::relative_target_span;

mod rule_set;

pub(super) fn decode(
    bytes: &[u8],
    span: ByteSpan,
    reader: &mut Reader<'_>,
) -> Decoded<ChainedContextPositioning> {
    let coverage_position = reader.offset();
    let coverage_offset = reader.u16()?;
    let backtrack_position = reader.offset();
    let backtrack_offset = reader.u16()?;
    let input_position = reader.offset();
    let input_offset = reader.u16()?;
    let lookahead_position = reader.offset();
    let lookahead_offset = reader.u16()?;
    let set_count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(set_count));
    for _ in 0..set_count {
        let position = reader.offset();
        offsets.push((position, reader.u16()?));
    }
    let minimum_position = reader.offset();
    let coverage = decode_coverage(
        bytes,
        span,
        coverage_position,
        coverage_offset,
        minimum_position,
    )?;
    let backtrack_classes = decode_classes(
        bytes,
        span,
        backtrack_position,
        backtrack_offset,
        minimum_position,
    )?;
    let input_classes =
        decode_classes(bytes, span, input_position, input_offset, minimum_position)?;
    let lookahead_classes = decode_classes(
        bytes,
        span,
        lookahead_position,
        lookahead_offset,
        minimum_position,
    )?;

    let mut rules = Vec::new();
    for (first_class, (position, offset)) in (0_u16..set_count).zip(offsets) {
        if offset == 0 {
            continue;
        }
        let target = relative_target_span(
            span,
            position,
            offset,
            minimum_position - span.start(),
            LayoutTarget::GposChainedContextRuleSet,
        )?;
        rules.extend(rule_set::decode(bytes, target, first_class)?);
    }
    Ok(ChainedContextPositioning::ClassRules {
        coverage,
        backtrack_classes,
        input_classes,
        lookahead_classes,
        rules,
    })
}

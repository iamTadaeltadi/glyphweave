//! `ChainContextSubst` format 1 exact-glyph rule-set decoding.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::ChainedContextSubstitution;
use crate::{gsub::decode::decode_coverage, layout_error, relative_target_span};

mod rule_set;

pub(super) fn decode(
    bytes: &[u8],
    span: ByteSpan,
    reader: &mut Reader<'_>,
) -> Decoded<ChainedContextSubstitution> {
    let coverage_position = reader.offset();
    let coverage_offset = reader.u16()?;
    let count_position = reader.offset();
    let count = reader.u16()?;
    let mut offsets = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
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
    if coverage.len() != offsets.len() {
        return Err(layout_error(
            count_position,
            LayoutCause::MismatchedGsubMappingCount {
                lookup_type: 6,
                inputs: coverage.len(),
                records: offsets.len(),
            },
        ));
    }
    let mut rules = Vec::new();
    for (first, (position, offset)) in coverage.glyphs().iter().copied().zip(offsets) {
        if offset == 0 {
            continue;
        }
        let target = relative_target_span(
            span,
            position,
            offset,
            minimum_position - span.start(),
            LayoutTarget::ChainedContextRuleSet,
        )?;
        rules.extend(rule_set::decode(bytes, target, first)?);
    }
    Ok(ChainedContextSubstitution::GlyphRules(rules))
}

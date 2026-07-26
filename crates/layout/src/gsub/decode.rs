//! Shared bounded targets used by GSUB subtable decoders.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutTarget};

use crate::{Coverage, relative_target_span};

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
        LayoutTarget::SubstitutionCoverage,
    )?;
    Coverage::decode(&bytes[..owner.end()], target.start())
}

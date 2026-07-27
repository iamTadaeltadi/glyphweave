//! Bounded decoding for `CursivePos` format 1.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTarget, Reader};

use super::{CursivePositioning, EntryExit};
use crate::{Coverage, gpos::Anchor, layout_error, relative_target_span};

impl CursivePositioning {
    /// Decodes a `CursivePos` subtable at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// Returns a span-aware Diagnostic for truncation, an unsupported format, an invalid
    /// Coverage or anchor offset, an unsupported anchor format, or an entry/exit record count
    /// that disagrees with the input Coverage.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error(
                format_position,
                LayoutCause::UnsupportedGposCursiveFormat { format },
            ));
        }
        let coverage_position = reader.offset();
        let coverage_offset = reader.u16()?;
        let count_position = reader.offset();
        let count = reader.u16()?;
        let mut offsets = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            let entry_position = reader.offset();
            let entry_offset = reader.u16()?;
            let exit_position = reader.offset();
            let exit_offset = reader.u16()?;
            offsets.push((entry_position, entry_offset, exit_position, exit_offset));
        }
        let minimum_position = reader.offset();

        let coverage_target = relative_target_span(
            span,
            coverage_position,
            coverage_offset,
            minimum_position - span.start(),
            LayoutTarget::GposCursiveCoverage,
        )?;
        let coverage = Coverage::decode(&bytes[..span.end()], coverage_target.start())?;
        if coverage.len() != offsets.len() {
            return Err(layout_error(
                count_position,
                LayoutCause::MismatchedGposEntryExitCount {
                    inputs: coverage.len(),
                    records: offsets.len(),
                },
            ));
        }

        let mut records = Vec::with_capacity(offsets.len());
        for (entry_position, entry_offset, exit_position, exit_offset) in offsets {
            let entry = decode_anchor(
                bytes,
                span,
                entry_position,
                entry_offset,
                minimum_position,
                LayoutTarget::GposCursiveEntryAnchor,
            )?;
            let exit = decode_anchor(
                bytes,
                span,
                exit_position,
                exit_offset,
                minimum_position,
                LayoutTarget::GposCursiveExitAnchor,
            )?;
            records.push(EntryExit::new(entry, exit));
        }
        Ok(Self::new(coverage, records))
    }
}

/// Resolves one optional entry or exit anchor, treating a null offset as an absent anchor.
fn decode_anchor(
    bytes: &[u8],
    owner: ByteSpan,
    position: usize,
    offset: u16,
    minimum_position: usize,
    target: LayoutTarget,
) -> Decoded<Option<Anchor>> {
    if offset == 0 {
        return Ok(None);
    }
    let anchor_span = relative_target_span(
        owner,
        position,
        offset,
        minimum_position - owner.start(),
        target,
    )?;
    Anchor::decode(&bytes[..owner.end()], anchor_span).map(Some)
}

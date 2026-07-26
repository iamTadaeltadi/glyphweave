use glyphweave_sfnt::{ByteSpan, Decoded, LayoutTableKind, LayoutTarget, Reader};

use super::LookupList;
use crate::{Lookup, relative_target_span};

impl LookupList {
    /// Decodes a GSUB or GPOS LookupList at the beginning of `span`.
    /// # Errors
    /// Returns a diagnostic for truncation or an invalid nested Lookup or relative target.
    pub fn decode(bytes: &[u8], span: ByteSpan, table: LayoutTableKind) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let count = reader.u16()?;
        let mut offsets = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            let position = reader.offset();
            offsets.push((position, reader.u16()?));
        }
        let minimum = reader.offset() - span.start();
        let mut lookups = Vec::with_capacity(offsets.len());
        for (position, offset) in offsets {
            let target =
                relative_target_span(span, position, offset, minimum, LayoutTarget::Lookup)?;
            lookups.push(Lookup::decode(bytes, target, table)?);
        }
        Ok(Self::new(lookups))
    }
}

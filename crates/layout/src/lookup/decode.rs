use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTableKind, LayoutTarget, Reader};

use super::{Lookup, extension};
use crate::{LookupFlags, layout_error, relative_target_span};

impl Lookup {
    /// Decodes one direct or extension Lookup at the beginning of `span`.
    /// # Errors
    /// Returns a diagnostic for invalid types, flags, offsets, wrappers, or truncated fields.
    pub fn decode(bytes: &[u8], span: ByteSpan, table: LayoutTableKind) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let type_position = reader.offset();
        let lookup_type = reader.u16()?;
        if !extension::is_valid_type(table, lookup_type) {
            return Err(layout_error(
                type_position,
                LayoutCause::InvalidLookupType { table, lookup_type },
            ));
        }
        let flags_position = reader.offset();
        let raw_flags = reader.u16()?;
        if raw_flags & 0x00e0 != 0 {
            return Err(layout_error(
                flags_position,
                LayoutCause::ReservedLookupFlags { flags: raw_flags },
            ));
        }
        let count_position = reader.offset();
        let count = reader.u16()?;
        if count == 0 {
            return Err(layout_error(
                count_position,
                LayoutCause::EmptyLookupSubtables { table, lookup_type },
            ));
        }
        let mut offsets = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            let position = reader.offset();
            offsets.push((position, reader.u16()?));
        }
        let filtering_set = (raw_flags & 0x0010 != 0)
            .then(|| reader.u16())
            .transpose()?;
        let minimum = reader.offset() - span.start();
        let mut subtables = Vec::with_capacity(offsets.len());
        for (position, offset) in offsets {
            subtables.push(relative_target_span(
                span,
                position,
                offset,
                minimum,
                LayoutTarget::LookupSubtable,
            )?);
        }
        let flags = LookupFlags::from_raw(raw_flags, filtering_set);
        if !extension::is_extension_type(table, lookup_type) {
            return Ok(Self::new(table, lookup_type, flags, subtables));
        }

        let (lookup_type, payloads) = extension::flatten_extensions(bytes, subtables, table)?;
        Ok(Self::new(table, lookup_type, flags, payloads))
    }
}

//! One semantic GPOS Lookup after extension flattening and subtable decoding.

use glyphweave_sfnt::{
    ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTableKind, LayoutTarget, Reader,
};

use crate::{GposSubtable, LookupFlags, layout_error, relative_target_span};

/// Ordered same-typed GPOS subtables with their shared processing qualifiers.
///
/// The `lookup_type` reports the effective positioning operation every subtable performs; a type 9
/// `ExtensionPos` Lookup is flattened during decoding, so it reports the wrapped real type rather
/// than the transport type.
///
/// ```
/// use glyphweave_layout::GposLookup;
/// use glyphweave_sfnt::ByteSpan;
///
/// let bytes = [
///     0x00, 0x01, // lookupType: single positioning
///     0x00, 0x00, // lookupFlag
///     0x00, 0x01, // subtableCount
///     0x00, 0x08, // subtableOffset[0]
///     0x00, 0x01, // SinglePos posFormat 1
///     0x00, 0x08, // coverageOffset
///     0x00, 0x04, // valueFormat: x advance
///     0xff, 0xec, // shared valueRecord.xAdvance = -20
///     0x00, 0x01, // Coverage format 1
///     0x00, 0x01, // glyph count
///     0x00, 0x05, // glyph id 5
/// ];
/// let span = ByteSpan::sized(0, bytes.len()).expect("span fits the buffer");
/// let lookup = GposLookup::decode(&bytes, span).expect("valid lookup");
/// assert_eq!(lookup.lookup_type(), 1);
/// assert_eq!(lookup.len(), 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GposLookup<G = GlyphId> {
    lookup_type: u16,
    flags: LookupFlags,
    subtables: Vec<GposSubtable<G>>,
}

impl GposLookup<GlyphId> {
    /// Decodes one GPOS Lookup table at the beginning of `span`.
    ///
    /// # Errors
    ///
    /// For reserved lookup flags, an empty subtable list, an out-of-range subtable offset, a
    /// truncated header, or any malformed effective or extension subtable.
    pub fn decode(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let lookup_type = reader.u16()?;
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
                LayoutCause::EmptyLookupSubtables {
                    table: LayoutTableKind::Positioning,
                    lookup_type,
                },
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
            let target = relative_target_span(
                span,
                position,
                offset,
                minimum,
                LayoutTarget::LookupSubtable,
            )?;
            subtables.push(GposSubtable::decode(bytes, target, lookup_type)?);
        }
        let effective_type = subtables
            .first()
            .map_or(lookup_type, GposSubtable::lookup_type);
        Ok(Self {
            lookup_type: effective_type,
            flags: LookupFlags::from_raw(raw_flags, filtering_set),
            subtables,
        })
    }
}

impl<G> GposLookup<G> {
    /// Returns the effective GPOS lookup type shared by every subtable.
    #[must_use]
    pub const fn lookup_type(&self) -> u16 {
        self.lookup_type
    }

    /// Returns the Lookup processing qualifiers.
    #[must_use]
    pub const fn flags(&self) -> LookupFlags {
        self.flags
    }

    /// Returns semantic subtables in shaping application order.
    #[must_use]
    pub fn subtables(&self) -> &[GposSubtable<G>] {
        &self.subtables
    }

    /// Returns the number of semantic subtables.
    #[must_use]
    pub fn len(&self) -> usize {
        self.subtables.len()
    }

    /// Returns whether no semantic subtable remains.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.subtables.is_empty()
    }

    /// Assembles a Lookup from already-decoded parts, preserving its effective type and flags.
    #[must_use]
    pub(crate) const fn from_parts(
        lookup_type: u16,
        flags: LookupFlags,
        subtables: Vec<GposSubtable<G>>,
    ) -> Self {
        Self {
            lookup_type,
            flags,
            subtables,
        }
    }
}

#[cfg(test)]
mod tests;

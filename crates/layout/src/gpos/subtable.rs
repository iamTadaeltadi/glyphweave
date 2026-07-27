//! One decoded effective GPOS subtable.

use glyphweave_sfnt::{
    ByteSpan, Decoded, GlyphId, LayoutCause, LayoutTableKind, LayoutTarget, Reader,
};

use crate::{
    ChainedContextPositioning, ContextPositioning, CursivePositioning, MarkLigaturePositioning,
    MarkToBasePositioning, MarkToMarkPositioning, PairPositioning, SinglePositioning, layout_error,
    layout_error_sized, relative_target_span_u32,
};

/// Every supported effective GPOS lookup operation after extension flattening.
///
/// A [`GposLookup`](crate::GposLookup) decodes each real payload span into one of these variants.
/// Lookup type 9 never appears here: an `ExtensionPos` wrapper is unwrapped during decoding, so a
/// value always names the real positioning operation it performs.
///
/// ```
/// use glyphweave_layout::{GposSubtable, SinglePositioning, ValueFormat, ValueRecord};
/// use glyphweave_sfnt::GlyphId;
///
/// let format = ValueFormat::from_bits(0x0004).expect("x advance");
/// let single = SinglePositioning::new(format, [(GlyphId::new(7), ValueRecord::new(0, 0, -30, 0))]);
/// let subtable = GposSubtable::Single(single);
/// assert_eq!(subtable.lookup_type(), 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GposSubtable<G = GlyphId> {
    /// Lookup type 1.
    Single(SinglePositioning<G>),
    /// Lookup type 2.
    Pair(PairPositioning<G>),
    /// Lookup type 3.
    Cursive(CursivePositioning<G>),
    /// Lookup type 4.
    MarkToBase(MarkToBasePositioning<G>),
    /// Lookup type 5.
    MarkToLigature(MarkLigaturePositioning<G>),
    /// Lookup type 6.
    MarkToMark(MarkToMarkPositioning<G>),
    /// Lookup type 7.
    Context(ContextPositioning<G>),
    /// Lookup type 8.
    ChainedContext(ChainedContextPositioning<G>),
}

impl GposSubtable<GlyphId> {
    /// Decodes the effective subtable named by `lookup_type`, unwrapping a type 9 extension.
    pub(super) fn decode(bytes: &[u8], span: ByteSpan, lookup_type: u16) -> Decoded<Self> {
        match lookup_type {
            1 => SinglePositioning::decode(bytes, span).map(Self::Single),
            2 => PairPositioning::decode(bytes, span).map(Self::Pair),
            3 => CursivePositioning::decode(bytes, span).map(Self::Cursive),
            4 => MarkToBasePositioning::decode(bytes, span).map(Self::MarkToBase),
            5 => MarkLigaturePositioning::decode(bytes, span).map(Self::MarkToLigature),
            6 => MarkToMarkPositioning::decode(bytes, span).map(Self::MarkToMark),
            7 => ContextPositioning::decode(bytes, span).map(Self::Context),
            8 => ChainedContextPositioning::decode(bytes, span).map(Self::ChainedContext),
            9 => Self::decode_extension(bytes, span),
            _ => Err(layout_error(
                span.start(),
                LayoutCause::InvalidLookupType {
                    table: LayoutTableKind::Positioning,
                    lookup_type,
                },
            )),
        }
    }

    /// Reads an `ExtensionPos` format 1 wrapper and decodes the real subtable it points to.
    fn decode_extension(bytes: &[u8], span: ByteSpan) -> Decoded<Self> {
        let mut reader = Reader::within(bytes, span)?;
        let format_position = reader.offset();
        let format = reader.u16()?;
        if format != 1 {
            return Err(layout_error_sized(
                format_position,
                2,
                LayoutCause::UnsupportedExtensionFormat {
                    table: LayoutTableKind::Positioning,
                    format,
                },
            ));
        }
        let type_position = reader.offset();
        let extension_type = reader.u16()?;
        if extension_type == LayoutTableKind::Positioning.extension_lookup_type() {
            return Err(layout_error_sized(
                type_position,
                2,
                LayoutCause::RecursiveGposExtension {
                    lookup_type: extension_type,
                },
            ));
        }
        if !(1..=8).contains(&extension_type) {
            return Err(layout_error_sized(
                type_position,
                2,
                LayoutCause::UnsupportedGposExtensionType {
                    lookup_type: extension_type,
                },
            ));
        }
        let offset_position = reader.offset();
        let offset = reader.u32()?;
        let payload = relative_target_span_u32(
            span,
            offset_position,
            4,
            offset,
            reader.offset() - span.start(),
            LayoutTarget::ExtensionSubtable,
        )?;
        Self::decode(bytes, payload, extension_type)
    }
}

impl<G> GposSubtable<G> {
    /// Returns the effective GPOS lookup type represented by this value.
    #[must_use]
    pub const fn lookup_type(&self) -> u16 {
        match self {
            Self::Single(_) => 1,
            Self::Pair(_) => 2,
            Self::Cursive(_) => 3,
            Self::MarkToBase(_) => 4,
            Self::MarkToLigature(_) => 5,
            Self::MarkToMark(_) => 6,
            Self::Context(_) => 7,
            Self::ChainedContext(_) => 8,
        }
    }
}

#[cfg(test)]
mod tests;

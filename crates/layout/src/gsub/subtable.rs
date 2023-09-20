//! One decoded effective GSUB subtable.

use glyphweave_sfnt::{ByteSpan, Decoded, LayoutCause, LayoutTableKind};

use crate::{
    AlternateSubstitution, ChainedContextSubstitution, ContextSubstitution, LigatureSubstitution,
    MultipleSubstitution, ReverseChainSingleSubstitution, SingleSubstitution, layout_error,
};

/// Every supported effective GSUB lookup operation after extension flattening.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GsubSubtable {
    /// Lookup type 1.
    Single(SingleSubstitution),
    /// Lookup type 2.
    Multiple(MultipleSubstitution),
    /// Lookup type 3.
    Alternate(AlternateSubstitution),
    /// Lookup type 4.
    Ligature(LigatureSubstitution),
    /// Lookup type 5.
    Context(ContextSubstitution),
    /// Lookup type 6.
    ChainedContext(ChainedContextSubstitution),
    /// Lookup type 8.
    ReverseChainSingle(ReverseChainSingleSubstitution),
}

impl GsubSubtable {
    pub(super) fn decode(bytes: &[u8], span: ByteSpan, lookup_type: u16) -> Decoded<Self> {
        match lookup_type {
            1 => SingleSubstitution::decode(bytes, span).map(Self::Single),
            2 => MultipleSubstitution::decode(bytes, span).map(Self::Multiple),
            3 => AlternateSubstitution::decode(bytes, span).map(Self::Alternate),
            4 => LigatureSubstitution::decode(bytes, span).map(Self::Ligature),
            5 => ContextSubstitution::decode(bytes, span).map(Self::Context),
            6 => ChainedContextSubstitution::decode(bytes, span).map(Self::ChainedContext),
            8 => ReverseChainSingleSubstitution::decode(bytes, span).map(Self::ReverseChainSingle),
            _ => Err(layout_error(
                span.start(),
                LayoutCause::InvalidLookupType {
                    table: LayoutTableKind::Substitution,
                    lookup_type,
                },
            )),
        }
    }

    /// Returns the effective GSUB lookup type represented by this value.
    #[must_use]
    pub const fn lookup_type(&self) -> u16 {
        match self {
            Self::Single(_) => 1,
            Self::Multiple(_) => 2,
            Self::Alternate(_) => 3,
            Self::Ligature(_) => 4,
            Self::Context(_) => 5,
            Self::ChainedContext(_) => 6,
            Self::ReverseChainSingle(_) => 8,
        }
    }

    /// Returns whether this subtable contains no semantic rule or mapping.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Single(value) => value.is_empty(),
            Self::Multiple(value) => value.is_empty(),
            Self::Alternate(value) => value.is_empty(),
            Self::Ligature(value) => value.is_empty(),
            Self::Context(value) => value.is_empty(),
            Self::ChainedContext(value) => value.is_empty(),
            Self::ReverseChainSingle(value) => value.is_empty(),
        }
    }
}

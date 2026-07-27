//! OpenType layout-specific diagnostic causes.

use core::fmt;

mod display;

/// The OpenType layout table family that assigns meaning to a lookup type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutTableKind {
    /// Glyph substitution through GSUB.
    Substitution,
    /// Glyph positioning through GPOS.
    Positioning,
}

impl LayoutTableKind {
    /// Returns the greatest lookup type defined by this table family.
    #[must_use]
    pub const fn maximum_lookup_type(self) -> u16 {
        match self {
            Self::Substitution => 8,
            Self::Positioning => 9,
        }
    }

    /// Returns the transport-only extension lookup type for this table family.
    #[must_use]
    pub const fn extension_lookup_type(self) -> u16 {
        match self {
            Self::Substitution => 7,
            Self::Positioning => 9,
        }
    }
}

impl fmt::Display for LayoutTableKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Substitution => "GSUB",
            Self::Positioning => "GPOS",
        })
    }
}

/// The tag-ordered layout list involved in a validation failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutListKind {
    /// The `ScriptRecord` array of a `ScriptList`.
    Script,
    /// The `LangSysRecord` array of a Script table.
    Language,
    /// The `FeatureRecord` array of a `FeatureList`.
    Feature,
}

impl fmt::Display for LayoutListKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Script => "script",
            Self::Language => "language",
            Self::Feature => "feature",
        })
    }
}

/// The relative-offset target involved in a layout validation failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutTarget {
    /// A Lookup table referenced by `LookupList`.
    Lookup,
    /// A direct or extension-wrapper Lookup subtable.
    LookupSubtable,
    /// The real payload referenced by an extension wrapper.
    ExtensionSubtable,
    /// The Coverage table selecting substitution inputs.
    SubstitutionCoverage,
    /// A `MultipleSubst` Sequence table.
    SubstitutionSequence,
    /// An `AlternateSubst` `AlternateSet` table.
    AlternateSet,
    /// A `LigatureSubst` `LigatureSet` table.
    LigatureSet,
    /// A Ligature table within a `LigatureSet`.
    Ligature,
    /// A contextual glyph Class Definition table.
    ContextClassDefinition,
    /// A `ContextSubst` rule-set table.
    ContextRuleSet,
    /// A `ContextSubst` rule table.
    ContextRule,
    /// A `ChainContextSubst` rule-set table.
    ChainedContextRuleSet,
    /// A `ChainContextSubst` rule table.
    ChainedContextRule,
    /// A Script table referenced by `ScriptList`.
    Script,
    /// The default Language System table of a Script.
    DefaultLanguageSystem,
    /// A named Language System table referenced by a Script.
    LanguageSystem,
    /// A Feature table referenced by `FeatureList`.
    Feature,
    /// A tag-specific Feature Parameters table.
    FeatureParameters,
    /// The Coverage table selecting GPOS single-positioning inputs.
    SinglePositioningCoverage,
    /// The Coverage table selecting the first glyph of a `PairPos` subtable.
    PairPositioningCoverage,
    /// A `PairPos` format 1 `PairSet` table.
    PairSet,
    /// A `PairPos` format 2 glyph Class Definition table.
    PairPositioningClassDefinition,
    /// The Coverage table selecting cursively positioned glyphs.
    GposCursiveCoverage,
    /// A `CursivePos` entry Anchor table.
    GposCursiveEntryAnchor,
    /// A `CursivePos` exit Anchor table.
    GposCursiveExitAnchor,
    /// A format 3 Anchor device or variation-index table.
    GposAnchorDevice,
}

impl fmt::Display for LayoutTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Lookup => "Lookup",
            Self::LookupSubtable => "Lookup subtable",
            Self::ExtensionSubtable => "extension subtable",
            Self::SubstitutionCoverage => "substitution Coverage",
            Self::SubstitutionSequence => "substitution Sequence",
            Self::AlternateSet => "AlternateSet",
            Self::LigatureSet => "LigatureSet",
            Self::Ligature => "Ligature",
            Self::ContextClassDefinition => "context Class Definition",
            Self::ContextRuleSet => "ContextRuleSet",
            Self::ContextRule => "ContextRule",
            Self::ChainedContextRuleSet => "ChainContextRuleSet",
            Self::ChainedContextRule => "ChainContextRule",
            Self::Script => "Script",
            Self::DefaultLanguageSystem => "default Language System",
            Self::LanguageSystem => "Language System",
            Self::Feature => "Feature",
            Self::FeatureParameters => "Feature Parameters",
            Self::SinglePositioningCoverage => "single positioning Coverage",
            Self::PairPositioningCoverage => "PairPos Coverage",
            Self::PairSet => "PairSet",
            Self::PairPositioningClassDefinition => "PairPos Class Definition",
            Self::GposCursiveCoverage => "cursive Coverage",
            Self::GposCursiveEntryAnchor => "cursive entry Anchor",
            Self::GposCursiveExitAnchor => "cursive exit Anchor",
            Self::GposAnchorDevice => "Anchor device",
        })
    }
}

/// A format or invariant failure in a shared OpenType layout structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LayoutCause {
    /// A Script, Language System, or Feature record used a malformed Tag.
    InvalidLayoutTag {
        /// The list containing the record.
        list: LayoutListKind,
        /// The malformed Tag.
        tag: crate::Tag,
    },
    /// A tagged record array decreased instead of remaining alphabetically ordered.
    UnsortedLayoutTags {
        /// The list containing the records.
        list: LayoutListKind,
        /// The preceding Tag.
        previous: crate::Tag,
        /// The decreasing Tag.
        current: crate::Tag,
    },
    /// A uniquely keyed tagged record array repeated a Tag.
    DuplicateLayoutTag {
        /// The list containing the records.
        list: LayoutListKind,
        /// The repeated Tag.
        tag: crate::Tag,
    },
    /// An Offset16 or Offset32 target was null, inside owner records, or outside its container.
    InvalidLayoutOffset {
        /// The kind of structure being targeted.
        target: LayoutTarget,
        /// The stored relative offset.
        offset: u32,
        /// The first legal relative target byte.
        minimum: usize,
        /// The containing structure's byte length.
        container_length: usize,
    },
    /// A Language System table used the reserved lookup-order offset.
    ReservedLookupOrderOffset {
        /// The nonzero reserved value.
        offset: u16,
    },
    /// A GSUB or GPOS Lookup used a type outside its defined enumeration.
    InvalidLookupType {
        /// The table family assigning meaning to the type.
        table: LayoutTableKind,
        /// The unrecognized lookup type.
        lookup_type: u16,
    },
    /// A family-specific decoder received a Lookup from the other layout table.
    UnexpectedLayoutTableKind {
        /// The family required by the decoder.
        expected: LayoutTableKind,
        /// The family carried by the Lookup.
        actual: LayoutTableKind,
    },
    /// A Lookup declared no subtables.
    EmptyLookupSubtables {
        /// The containing layout table family.
        table: LayoutTableKind,
        /// The declared lookup type.
        lookup_type: u16,
    },
    /// A Lookup set bits reserved in the shared lookup-flag word.
    ReservedLookupFlags {
        /// The complete raw flag word.
        flags: u16,
    },
    /// An extension wrapper used a format other than format 1.
    UnsupportedExtensionFormat {
        /// The containing layout table family.
        table: LayoutTableKind,
        /// The unsupported wrapper format.
        format: u16,
    },
    /// An extension wrapper targeted its own transport lookup type.
    RecursiveExtensionLookupType {
        /// The containing layout table family.
        table: LayoutTableKind,
        /// The recursively declared extension type.
        lookup_type: u16,
    },
    /// Extension wrappers in one Lookup declared different real types.
    MismatchedExtensionLookupTypes {
        /// The containing layout table family.
        table: LayoutTableKind,
        /// The real type declared by the first wrapper.
        expected: u16,
        /// The different real type declared by a later wrapper.
        actual: u16,
    },
    /// A supported GSUB lookup type used an unknown subtable format.
    UnsupportedGsubFormat {
        /// The effective GSUB lookup type.
        lookup_type: u16,
        /// The unrecognized format selector.
        format: u16,
    },
    /// A parallel GSUB record array did not match its input Coverage length.
    MismatchedGsubMappingCount {
        /// The effective GSUB lookup type.
        lookup_type: u16,
        /// The number of covered input glyphs.
        inputs: usize,
        /// The number of output or rule-set records.
        records: usize,
    },
    /// A Ligature table declared fewer than two input components.
    InvalidLigatureComponentCount {
        /// The declared count, including the implicit first Coverage glyph.
        count: u16,
    },
    /// A contextual GSUB rule declared an empty input sequence.
    EmptyContextInput {
        /// `ContextSubst` type 5 or `ChainContextSubst` type 6.
        lookup_type: u16,
    },
    /// A contextual action targeted a position outside its input sequence.
    InvalidSubstitutionSequenceIndex {
        /// `ContextSubst` type 5 or `ChainContextSubst` type 6.
        lookup_type: u16,
        /// The zero-based sequence index in the action record.
        sequence_index: u16,
        /// The contextual input sequence length.
        input_count: u16,
    },
    /// A contextual action referenced a `LookupList` index that does not exist.
    InvalidContextLookupIndex {
        /// The referenced zero-based `LookupList` index.
        lookup_index: u16,
        /// The number of available Lookups.
        lookup_count: usize,
    },
    /// A retained layout reference has no entry in the output Glyph ID Remap.
    UnmappedLayoutGlyph {
        /// The layout table being rewritten.
        table: LayoutTableKind,
        /// The retained input Glyph ID missing from the Remap.
        glyph: crate::GlyphId,
    },
    /// A Coverage table used an unknown format selector.
    UnsupportedCoverageFormat {
        /// The unrecognized selector.
        format: u16,
    },
    /// Coverage format 1 did not list glyph IDs in strict ascending order.
    NonIncreasingCoverageGlyph {
        /// The preceding glyph ID.
        previous: crate::GlyphId,
        /// The repeated or decreasing glyph ID.
        current: crate::GlyphId,
    },
    /// A Coverage range ended before it began.
    InvalidCoverageRange {
        /// The first glyph ID in the range.
        start: crate::GlyphId,
        /// The last glyph ID in the range.
        end: crate::GlyphId,
    },
    /// A Coverage range began at or before the preceding range ended.
    OverlappingCoverageRange {
        /// The preceding range's final glyph ID.
        previous_end: crate::GlyphId,
        /// The next range's first glyph ID.
        start: crate::GlyphId,
    },
    /// A format 2 range did not continue the sequential Coverage Index space.
    DiscontinuousCoverageIndex {
        /// The index derived from preceding ranges.
        expected: u32,
        /// The index stored in the range record.
        actual: u16,
    },
    /// A Class Definition table used an unknown format selector.
    UnsupportedClassDefinitionFormat {
        /// The unrecognized selector.
        format: u16,
    },
    /// A format 1 Class Definition range extends beyond glyph ID 65535.
    ClassDefinitionGlyphRangeOverflow {
        /// The first glyph ID in the dense range.
        start: crate::GlyphId,
        /// The number of class values in the range.
        count: u16,
    },
    /// A Class Definition range ended before it began.
    InvalidClassDefinitionRange {
        /// The first glyph ID in the range.
        start: crate::GlyphId,
        /// The last glyph ID in the range.
        end: crate::GlyphId,
    },
    /// A Class Definition range began at or before the preceding range ended.
    OverlappingClassDefinitionRange {
        /// The preceding range's final glyph ID.
        previous_end: crate::GlyphId,
        /// The next range's first glyph ID.
        start: crate::GlyphId,
    },
    /// A GPOS `SinglePos` subtable used an unknown format selector.
    UnsupportedGposSingleFormat {
        /// The unrecognized format selector.
        format: u16,
    },
    /// A GPOS `SinglePos` value format set a bit reserved by the specification.
    ReservedGposSingleValueFormat {
        /// The complete raw value-format word.
        format: u16,
    },
    /// A GPOS `SinglePos` format 2 value-record array did not match its Coverage length.
    MismatchedGposSingleValueCount {
        /// The number of covered glyphs.
        inputs: usize,
        /// The number of encoded value records.
        records: usize,
    },
    /// A GPOS `PairPos` subtable used an unknown format selector.
    UnsupportedGposPairFormat {
        /// The unrecognized selector.
        format: u16,
    },
    /// A GPOS `PairPos` value record used a value format that sets reserved bits.
    ReservedGposPairValueFormat {
        /// Which record of the pair carried the format: 1 or 2.
        record: u8,
        /// The complete raw value-format word.
        format: u16,
    },
    /// A GPOS `PairPos` format 1 `PairSet` array did not match its Coverage length.
    MismatchedGposPairSetCount {
        /// The number of covered first glyphs.
        inputs: usize,
        /// The number of `PairSet` offsets.
        sets: usize,
    },
    /// A GPOS Anchor table used a format outside the defined 1 through 3 enumeration.
    UnsupportedGposAnchorFormat {
        /// The unrecognized anchor format selector.
        format: u16,
    },
    /// A GPOS `CursivePos` subtable used a format other than format 1.
    UnsupportedGposCursiveFormat {
        /// The unrecognized `CursivePos` format selector.
        format: u16,
    },
    /// A GPOS `CursivePos` entry/exit record array disagreed with its input Coverage length.
    MismatchedGposEntryExitCount {
        /// The number of covered input glyphs.
        inputs: usize,
        /// The number of entry/exit records.
        records: usize,
    },
}

impl fmt::Display for LayoutCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::InvalidLayoutTag { list, tag } => {
                write!(f, "invalid {list} tag {tag:?}")
            }
            Self::UnsortedLayoutTags {
                list,
                previous,
                current,
            } => write!(f, "{list} tag {current} follows larger tag {previous}"),
            Self::DuplicateLayoutTag { list, tag } => {
                write!(f, "duplicate {list} tag {tag}")
            }
            Self::InvalidLayoutOffset {
                target,
                offset,
                minimum,
                container_length,
            } => write!(
                f,
                "{target} offset {offset} is outside legal range {minimum}..{container_length}"
            ),
            Self::ReservedLookupOrderOffset { offset } => {
                write!(f, "reserved lookup-order offset must be zero, not {offset}")
            }
            Self::InvalidLookupType { .. }
            | Self::UnexpectedLayoutTableKind { .. }
            | Self::EmptyLookupSubtables { .. }
            | Self::ReservedLookupFlags { .. }
            | Self::UnsupportedExtensionFormat { .. }
            | Self::RecursiveExtensionLookupType { .. }
            | Self::MismatchedExtensionLookupTypes { .. } => display::lookup(self, f),
            Self::UnsupportedGsubFormat { .. }
            | Self::MismatchedGsubMappingCount { .. }
            | Self::InvalidLigatureComponentCount { .. }
            | Self::EmptyContextInput { .. }
            | Self::InvalidSubstitutionSequenceIndex { .. }
            | Self::InvalidContextLookupIndex { .. }
            | Self::UnmappedLayoutGlyph { .. } => display::gsub(self, f),
            Self::UnsupportedCoverageFormat { format } => {
                write!(f, "unsupported Coverage format {format}")
            }
            Self::NonIncreasingCoverageGlyph { previous, current } => write!(
                f,
                "Coverage glyph {current} does not strictly follow {previous}"
            ),
            Self::InvalidCoverageRange { start, end } => {
                write!(f, "Coverage range {start}..={end} has reversed bounds")
            }
            Self::OverlappingCoverageRange {
                previous_end,
                start,
            } => write!(
                f,
                "Coverage range beginning at {start} overlaps preceding end {previous_end}"
            ),
            Self::DiscontinuousCoverageIndex { expected, actual } => write!(
                f,
                "Coverage range index {actual} does not continue at {expected}"
            ),
            Self::UnsupportedClassDefinitionFormat { format } => {
                write!(f, "unsupported Class Definition format {format}")
            }
            Self::ClassDefinitionGlyphRangeOverflow { start, count } => write!(
                f,
                "Class Definition range beginning at {start} with {count} value(s) exceeds glyph ID 65535"
            ),
            Self::InvalidClassDefinitionRange { start, end } => write!(
                f,
                "Class Definition range {start}..={end} has reversed bounds"
            ),
            Self::OverlappingClassDefinitionRange {
                previous_end,
                start,
            } => write!(
                f,
                "Class Definition range beginning at {start} overlaps preceding end {previous_end}"
            ),
            Self::UnsupportedGposSingleFormat { format } => {
                write!(f, "unsupported GPOS SinglePos format {format}")
            }
            Self::ReservedGposSingleValueFormat { format } => write!(
                f,
                "GPOS SinglePos value format {format:#06x} sets a reserved bit"
            ),
            Self::MismatchedGposSingleValueCount { inputs, records } => write!(
                f,
                "GPOS SinglePos covers {inputs} glyph(s) but declares {records} value record(s)"
            ),
            Self::UnsupportedGposPairFormat { format } => {
                write!(f, "unsupported GPOS PairPos format {format}")
            }
            Self::ReservedGposPairValueFormat { record, format } => write!(
                f,
                "GPOS PairPos value record {record} format {format:#06x} sets reserved bits"
            ),
            Self::MismatchedGposPairSetCount { inputs, sets } => write!(
                f,
                "GPOS PairPos has {inputs} covered glyph(s) but {sets} pair set(s)"
            ),
            Self::UnsupportedGposAnchorFormat { format } => {
                write!(f, "unsupported GPOS Anchor format {format}")
            }
            Self::UnsupportedGposCursiveFormat { format } => {
                write!(f, "unsupported GPOS CursivePos format {format}")
            }
            Self::MismatchedGposEntryExitCount { inputs, records } => write!(
                f,
                "GPOS CursivePos has {inputs} input glyph(s) but {records} entry/exit record(s)"
            ),
        }
    }
}

#[cfg(test)]
mod tests;

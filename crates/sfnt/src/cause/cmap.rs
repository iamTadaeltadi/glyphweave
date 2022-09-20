//! Typed causes produced while decoding character-map subtables.

mod display;

/// The kind of nested mapping table referenced by a format-14 selector record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cmap14TableKind {
    /// A compressed list of base-scalar ranges that delegate to the ordinary Character Map.
    Default,
    /// A list of base scalars with explicit non-default Glyph IDs.
    NonDefault,
}

/// A format-specific cause produced by a `cmap` decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CmapCause {
    /// A `cmap` subtable used a format other than the decoder expected.
    UnsupportedCmapFormat {
        /// The format required by the selected decoder.
        expected: u16,
        /// The encoded subtable format.
        found: u16,
    },
    /// A format-4 declared length cannot contain its arrays or exceeds available bytes.
    InvalidCmapLength {
        /// The encoded byte length.
        declared: u16,
        /// The minimum byte length implied by the segment count.
        minimum: usize,
        /// The bytes available from the subtable offset.
        available: usize,
    },
    /// A format-4 subtable declared an odd byte length.
    OddCmapLength {
        /// The odd encoded byte length.
        declared: u16,
    },
    /// A format-12 declared length cannot contain its header or exceeds available bytes.
    InvalidCmap12Length {
        /// The encoded byte length.
        declared: u32,
        /// The fixed minimum byte length.
        minimum: usize,
        /// The bytes available from the subtable offset.
        available: usize,
    },
    /// A format-12 declared length disagrees with its fixed-width group count.
    Cmap12LengthMismatch {
        /// The encoded byte length.
        declared: u32,
        /// The exact byte length implied by the group count.
        expected: u64,
        /// The encoded number of groups.
        groups: u32,
    },
    /// The required format-12 reserved word was nonzero.
    InvalidCmap12Reserved {
        /// The encoded reserved value.
        found: u16,
    },
    /// A format-12 group is inverted, not ordered, or overlaps its predecessor.
    InvalidCmap12Group {
        /// The zero-based group index.
        index: u32,
        /// The group's inclusive start character code.
        start: u32,
        /// The group's inclusive end character code.
        end: u32,
        /// The preceding group's end code, when this is not the first group.
        previous_end: Option<u32>,
    },
    /// A sequential format-12 mapping cannot be represented as a Glyph ID.
    Cmap12GlyphOutOfRange {
        /// The zero-based group index.
        group: u32,
        /// The character code whose mapping first exceeds the Glyph ID domain.
        codepoint: u32,
        /// The calculated glyph value.
        glyph: u64,
    },
    /// A cmap entry maps a non-scalar Unicode value to a nonzero glyph.
    CmapNonScalarMapping {
        /// The cmap subtable format.
        format: u16,
        /// The invalid Unicode value.
        codepoint: u32,
        /// The invalid nonzero Glyph ID.
        glyph: crate::GlyphId,
    },
    /// A format-4 `segCountX2` field is zero or odd.
    InvalidCmapSegmentCount {
        /// The invalid encoded value.
        encoded: u16,
    },
    /// Format-4 search fields disagree with values derived from the segment count.
    InvalidCmapSearchParameters {
        /// The derived `(searchRange, entrySelector, rangeShift)` values.
        expected: [u16; 3],
        /// The values stored in the subtable.
        actual: [u16; 3],
    },
    /// The required format-4 reserved padding word was nonzero.
    InvalidCmapReservedPad {
        /// The encoded reserved value.
        found: u16,
    },
    /// A format-4 segment is inverted, not ordered, or overlaps its predecessor.
    InvalidCmapSegment {
        /// The zero-based segment index.
        index: u16,
        /// The segment's inclusive start code.
        start: u16,
        /// The segment's inclusive end code.
        end: u16,
        /// The preceding segment's end code, when this is not the first segment.
        previous_end: Option<u16>,
    },
    /// The final format-4 segment is not the required `0xFFFF..=0xFFFF` range.
    InvalidCmapSentinelRange {
        /// The final segment start code.
        start: u16,
        /// The final segment end code.
        end: u16,
    },
    /// The required final format-4 sentinel maps `0xFFFF` to a nonzero glyph.
    InvalidCmapSentinel {
        /// The final segment start code.
        start: u16,
        /// The final segment end code.
        end: u16,
        /// The resolved sentinel Glyph ID.
        glyph: crate::GlyphId,
    },
    /// A format-4 range offset targets a word outside `glyphIdArray`.
    InvalidCmapRangeOffset {
        /// The zero-based segment index.
        segment: u16,
        /// The character code being resolved.
        codepoint: u16,
        /// The calculated absolute byte offset.
        target: usize,
        /// The legal glyph-array range.
        glyph_array: crate::ByteSpan,
    },
    /// A format-4 entry maps a UTF-16 surrogate to a nonzero glyph.
    CmapSurrogateMapping {
        /// The surrogate code unit.
        codepoint: u16,
        /// The invalid nonzero Glyph ID.
        glyph: crate::GlyphId,
    },
    /// A format-14 declared length cannot contain its header or exceeds available bytes.
    InvalidCmap14Length {
        /// The encoded byte length.
        declared: u32,
        /// The minimum byte length required at this point in decoding.
        minimum: usize,
        /// The bytes available from the subtable offset.
        available: usize,
    },
    /// The format-14 selector count cannot fit in the declared record array.
    Cmap14SelectorCountOutOfBounds {
        /// The encoded number of Variation Selector records.
        count: u32,
        /// The greatest record count that fits in the declared length.
        maximum: usize,
    },
    /// A format-14 selector is not typed, strictly ordered, or unique.
    InvalidCmap14VariationSelector {
        /// The zero-based selector-record index.
        index: u32,
        /// The encoded 24-bit selector value.
        selector: u32,
        /// The preceding selector value, when this is not the first record.
        previous: Option<u32>,
    },
    /// A nonzero format-14 nested-table offset does not locate a valid count field.
    InvalidCmap14Offset {
        /// The selector whose record contains the offset.
        selector: u32,
        /// The referenced nested-table kind.
        kind: Cmap14TableKind,
        /// The encoded offset relative to the format-14 subtable.
        offset: u32,
        /// The first legal relative target byte after the selector records.
        records_end: usize,
        /// The declared format-14 byte length.
        declared_length: usize,
    },
    /// A nested format-14 entry count cannot fit before the declared subtable end.
    Cmap14EntryCountOutOfBounds {
        /// The selector whose record references the nested table.
        selector: u32,
        /// The referenced nested-table kind.
        kind: Cmap14TableKind,
        /// The encoded entry count.
        count: u32,
        /// The greatest entry count that fits at the referenced offset.
        maximum: usize,
    },
    /// A default UVS range is not scalar, ordered, disjoint, or 24-bit bounded.
    InvalidCmap14DefaultRange {
        /// The selector associated with the range.
        selector: u32,
        /// The zero-based range index within its nested table.
        index: u32,
        /// The encoded inclusive range start.
        start: u32,
        /// The calculated inclusive range end.
        end: u32,
        /// The preceding range end, when this is not the first range.
        previous_end: Option<u32>,
    },
    /// A non-default UVS mapping is not a scalar, strictly ordered, or unique.
    InvalidCmap14NonDefaultMapping {
        /// The selector associated with the mapping.
        selector: u32,
        /// The zero-based mapping index within its nested table.
        index: u32,
        /// The encoded base character value.
        codepoint: u32,
        /// The preceding base value, when this is not the first mapping.
        previous: Option<u32>,
    },
    /// One sequence is listed in both default and non-default tables for its selector.
    ConflictingCmap14Mapping {
        /// The shared Variation Selector value.
        selector: u32,
        /// The base character listed in both mapping categories.
        codepoint: u32,
    },
    /// Two format-14 nested tables partially overlap or alias as different kinds.
    OverlappingCmap14Tables {
        /// The first target in absolute byte order.
        first: (Cmap14TableKind, crate::ByteSpan),
        /// The next target that conflicts with it.
        second: (Cmap14TableKind, crate::ByteSpan),
    },
}

//! GPOS positioning whose glyph references are all in emitted index space.

use glyphweave_sfnt::NewGlyphId;

use crate::{GposSubtable, LookupFlags};

/// One remapped GPOS Lookup: its effective type, flags, and emitted-space subtables.
///
/// A Lookup keeps its original `LookupList` slot even when remapping empties every subtable, so the
/// contextual `SequenceLookupRecord` indices resolved elsewhere in the pipeline stay valid.
///
/// ```
/// use glyphweave_layout::{GposLookupList, GposSubtable};
/// use glyphweave_sfnt::{ByteSpan, GlyphId, NewGlyphId};
///
/// let bytes = [
///     0x00, 0x01, // lookupCount
///     0x00, 0x04, // lookupOffset[0]
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
/// let list = GposLookupList::decode(&bytes, span).expect("valid lookup list");
/// let remapped = list.remap(|glyph| (glyph == GlyphId::new(5)).then_some(NewGlyphId::new(2)));
/// let lookup = remapped.lookup(0).expect("index preserved");
/// assert_eq!(lookup.lookup_type(), 1);
/// assert_eq!(lookup.len(), 1);
/// let GposSubtable::Single(single) = &lookup.subtables()[0] else { panic!("single") };
/// assert!(single.adjustment(NewGlyphId::new(2)).is_some());
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemappedGposLookup {
    lookup_type: u16,
    flags: LookupFlags,
    subtables: Vec<GposSubtable<NewGlyphId>>,
}

impl RemappedGposLookup {
    /// Builds a remapped Lookup from its retained type, flags, and emitted-space subtables.
    #[must_use]
    pub(super) const fn new(
        lookup_type: u16,
        flags: LookupFlags,
        subtables: Vec<GposSubtable<NewGlyphId>>,
    ) -> Self {
        Self {
            lookup_type,
            flags,
            subtables,
        }
    }

    /// Returns the effective GPOS lookup type shared by every subtable.
    #[must_use]
    pub const fn lookup_type(&self) -> u16 {
        self.lookup_type
    }

    /// Returns the Lookup processing qualifiers, carried over unchanged from the input.
    #[must_use]
    pub const fn flags(&self) -> LookupFlags {
        self.flags
    }

    /// Returns the remapped subtables in shaping application order.
    #[must_use]
    pub fn subtables(&self) -> &[GposSubtable<NewGlyphId>] {
        &self.subtables
    }

    /// Returns the number of remapped subtables.
    #[must_use]
    pub fn len(&self) -> usize {
        self.subtables.len()
    }

    /// Returns whether the Lookup retains no subtable.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.subtables.is_empty()
    }
}

/// A remapped GPOS `LookupList` that cannot contain input-space Glyph IDs.
///
/// Every Lookup keeps its original index so nested contextual references stay resolvable; a Lookup
/// whose glyphs were all dropped by the subset survives as an emptied placeholder.
///
/// ```
/// use glyphweave_layout::RemappedGpos;
///
/// let positioning = RemappedGpos::default();
/// assert!(positioning.is_empty());
/// assert_eq!(positioning.len(), 0);
/// assert!(positioning.lookup(0).is_none());
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RemappedGpos {
    lookups: Vec<RemappedGposLookup>,
}

impl RemappedGpos {
    /// Builds a remapped `LookupList` from index-parallel remapped Lookups.
    #[must_use]
    pub(super) const fn from_lookups(lookups: Vec<RemappedGposLookup>) -> Self {
        Self { lookups }
    }

    /// Returns one remapped Lookup at its stable original `LookupList` index.
    #[must_use]
    pub fn lookup(&self, index: u16) -> Option<&RemappedGposLookup> {
        self.lookups.get(usize::from(index))
    }

    /// Returns every remapped Lookup in original index order.
    #[must_use]
    pub fn lookups(&self) -> &[RemappedGposLookup] {
        &self.lookups
    }

    /// Returns the number of original Lookup slots, including emptied placeholders.
    #[must_use]
    pub fn len(&self) -> usize {
        self.lookups.len()
    }

    /// Returns whether the original `LookupList` had no Lookups.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lookups.is_empty()
    }
}

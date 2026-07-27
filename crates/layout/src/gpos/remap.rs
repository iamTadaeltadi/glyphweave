//! Total conversion of GPOS glyph references into emitted Glyph ID space.

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::{
    ClassDefinition, Coverage, GposLookup, GposLookupList, GposSubtable, RemappedGpos,
    RemappedGposLookup,
};

mod chained;
mod context;
mod cursive;
mod mark;
mod pair;
mod single;

#[cfg(test)]
mod tests;

impl GposLookupList<GlyphId> {
    /// Rewrites every glyph reference into emitted Glyph ID space, dropping absent glyphs.
    ///
    /// GPOS positioning never extends the glyph set, so any Coverage entry, class assignment, or
    /// per-glyph positioning record naming a glyph absent from `mapping` is discarded rather than
    /// remapped. Every Lookup keeps its original index and every subtable its slot, so contextual
    /// `SequenceLookupRecord` indices resolved later in the pipeline stay valid.
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
    /// let GposSubtable::Single(single) = &lookup.subtables()[0] else { panic!("single") };
    /// assert_eq!(single.adjustment(NewGlyphId::new(2)).map(|record| record.x_advance()), Some(-20));
    /// assert!(single.adjustment(NewGlyphId::new(5)).is_none());
    /// ```
    #[must_use]
    pub fn remap(&self, mut mapping: impl FnMut(GlyphId) -> Option<NewGlyphId>) -> RemappedGpos {
        let lookups = self
            .lookups()
            .iter()
            .map(|lookup| remap_lookup(lookup, &mut mapping))
            .collect();
        RemappedGpos::from_lookups(lookups)
    }
}

fn remap_lookup(
    input: &GposLookup,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> RemappedGposLookup {
    let subtables = input
        .subtables()
        .iter()
        .map(|subtable| remap_subtable(subtable, mapping))
        .collect();
    RemappedGposLookup::new(input.lookup_type(), input.flags(), subtables)
}

fn remap_subtable(
    input: &GposSubtable,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> GposSubtable<NewGlyphId> {
    match input {
        GposSubtable::Single(value) => GposSubtable::Single(single::positioning(value, mapping)),
        GposSubtable::Pair(value) => GposSubtable::Pair(pair::positioning(value, mapping)),
        GposSubtable::Cursive(value) => GposSubtable::Cursive(cursive::positioning(value, mapping)),
        GposSubtable::MarkToBase(value) => GposSubtable::MarkToBase(mark::to_base(value, mapping)),
        GposSubtable::MarkToLigature(value) => {
            GposSubtable::MarkToLigature(mark::to_ligature(value, mapping))
        }
        GposSubtable::MarkToMark(value) => GposSubtable::MarkToMark(mark::to_mark(value, mapping)),
        GposSubtable::Context(value) => GposSubtable::Context(context::positioning(value, mapping)),
        GposSubtable::ChainedContext(value) => {
            GposSubtable::ChainedContext(chained::positioning(value, mapping))
        }
    }
}

/// Rewrites a Coverage, dropping any glyph the subset does not retain.
pub(super) fn coverage(
    input: &Coverage,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Coverage<NewGlyphId> {
    input
        .glyphs()
        .iter()
        .filter_map(|&glyph| mapping(glyph))
        .collect()
}

/// Rewrites a Class Definition, dropping assignments for absent glyphs.
pub(super) fn classes(
    input: &ClassDefinition,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> ClassDefinition<NewGlyphId> {
    input
        .assignments()
        .filter_map(|(glyph, class)| mapping(glyph).map(|new| (new, class)))
        .collect()
}

/// Rewrites a glyph sequence, dropping the whole sequence if any glyph is absent.
pub(super) fn glyphs(
    input: &[GlyphId],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Option<Vec<NewGlyphId>> {
    input.iter().map(|&glyph| mapping(glyph)).collect()
}

/// Rewrites a Coverage and its Coverage-Index-parallel records together, dropping absent glyphs.
///
/// The surviving pairs are re-sorted into canonical emitted Coverage order so the returned records
/// stay aligned with the returned Coverage even when the remap reorders Glyph IDs.
pub(super) fn parallel<R: Clone>(
    coverage: &Coverage,
    records: &[R],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> (Coverage<NewGlyphId>, Vec<R>) {
    let mut paired: Vec<(NewGlyphId, R)> = coverage
        .glyphs()
        .iter()
        .zip(records)
        .filter_map(|(&glyph, record)| mapping(glyph).map(|new| (new, record.clone())))
        .collect();
    paired.sort_by_key(|entry| entry.0);
    let glyphs = paired.iter().map(|entry| entry.0).collect();
    let records = paired.into_iter().map(|entry| entry.1).collect();
    (glyphs, records)
}

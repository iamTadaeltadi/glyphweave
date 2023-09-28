//! One monotone application of a reachable semantic Lookup.

use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use crate::GsubLookup;

mod context;
mod direct;
mod feasible;

pub(super) fn lookup(
    lookup: &GsubLookup,
    glyphs: &mut BTreeSet<GlyphId>,
    lookups: &mut BTreeSet<u16>,
) {
    for subtable in lookup.subtables() {
        direct::apply(subtable, glyphs);
        context::apply(subtable, glyphs, lookups);
    }
}

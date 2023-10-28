//! Total conversion from retained input Glyph IDs to emitted Glyph IDs.

use glyphweave_sfnt::{Decoded, GlyphId, LayoutCause, LayoutTableKind, NewGlyphId};

use crate::{
    ClassDefinition, ContextRule, Coverage, GsubLookup, GsubLookupList, GsubSubtable, RemappedGsub,
    RetainedGsub, rewrite_error,
};

mod chained;
mod context;
mod direct;
mod ligature;
mod reverse;

impl RetainedGsub {
    /// Converts every retained glyph reference into emitted Glyph ID space.
    /// # Errors
    /// Returns a Rewrite-stage diagnostic if `mapping` is undefined for any retained glyph.
    pub fn remap(
        &self,
        mut mapping: impl FnMut(GlyphId) -> Option<NewGlyphId>,
    ) -> Decoded<RemappedGsub> {
        let lookups = self
            .lookup_list()
            .lookups()
            .iter()
            .map(|lookup| remap_lookup(lookup, &mut mapping))
            .collect::<Decoded<Vec<_>>>()?;
        Ok(RemappedGsub {
            lookups: GsubLookupList::from_lookups(lookups),
        })
    }
}

fn remap_lookup(
    input: &GsubLookup,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<GsubLookup<NewGlyphId>> {
    let subtables = input
        .subtables()
        .iter()
        .map(|subtable| remap_subtable(subtable, mapping))
        .collect::<Decoded<Vec<_>>>()?;
    Ok(GsubLookup::from_parts(
        input.lookup_type(),
        input.flags(),
        subtables,
    ))
}

fn remap_subtable(
    input: &GsubSubtable,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<GsubSubtable<NewGlyphId>> {
    match input {
        GsubSubtable::Single(value) => direct::single(value, mapping).map(GsubSubtable::Single),
        GsubSubtable::Multiple(value) => {
            direct::multiple(value, mapping).map(GsubSubtable::Multiple)
        }
        GsubSubtable::Alternate(value) => {
            direct::alternate(value, mapping).map(GsubSubtable::Alternate)
        }
        GsubSubtable::Ligature(value) => {
            ligature::substitution(value, mapping).map(GsubSubtable::Ligature)
        }
        GsubSubtable::Context(value) => {
            context::substitution(value, mapping).map(GsubSubtable::Context)
        }
        GsubSubtable::ChainedContext(value) => {
            chained::substitution(value, mapping).map(GsubSubtable::ChainedContext)
        }
        GsubSubtable::ReverseChainSingle(value) => {
            reverse::substitution(value, mapping).map(GsubSubtable::ReverseChainSingle)
        }
    }
}

pub(super) fn glyph(
    input: GlyphId,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<NewGlyphId> {
    mapping(input).ok_or_else(|| {
        rewrite_error(LayoutCause::UnmappedLayoutGlyph {
            table: LayoutTableKind::Substitution,
            glyph: input,
        })
    })
}

pub(super) fn coverage(
    input: &Coverage,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<Coverage<NewGlyphId>> {
    input
        .glyphs()
        .iter()
        .copied()
        .map(|input| glyph(input, mapping))
        .collect()
}

pub(super) fn classes(
    input: &ClassDefinition,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<ClassDefinition<NewGlyphId>> {
    let assignments = input
        .assignments()
        .map(|(input, class)| glyph(input, mapping).map(|output| (output, class)))
        .collect::<Decoded<Vec<_>>>()?;
    Ok(ClassDefinition::from_assignments(assignments))
}

pub(super) fn rule<T, U>(
    input: &ContextRule<T>,
    lookup_type: u16,
    mut predicate: impl FnMut(&T) -> Decoded<U>,
) -> Decoded<ContextRule<U>> {
    let backtrack = input
        .backtrack()
        .iter()
        .map(&mut predicate)
        .collect::<Decoded<Vec<_>>>()?;
    let mut predicates = input.input().iter();
    let first = predicates
        .next()
        .ok_or_else(|| rewrite_error(LayoutCause::EmptyContextInput { lookup_type }))?;
    let first = predicate(first)?;
    let remaining = predicates
        .map(&mut predicate)
        .collect::<Decoded<Vec<_>>>()?;
    let lookahead = input
        .lookahead()
        .iter()
        .map(predicate)
        .collect::<Decoded<Vec<_>>>()?;
    let records = input.records().to_vec();
    Ok(ContextRule::chained(
        backtrack, first, remaining, lookahead, records,
    ))
}

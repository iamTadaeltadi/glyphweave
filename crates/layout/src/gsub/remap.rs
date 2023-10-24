//! Total conversion from retained input Glyph IDs to emitted Glyph IDs.

use glyphweave_sfnt::{Decoded, GlyphId, LayoutCause, LayoutTableKind, NewGlyphId};

use crate::{ClassDefinition, ContextRule, Coverage, SubstitutionRecord, rewrite_error};

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
    let records = input
        .records()
        .iter()
        .copied()
        .collect::<Vec<SubstitutionRecord>>();
    Ok(ContextRule::chained(
        backtrack, first, remaining, lookahead, records,
    ))
}

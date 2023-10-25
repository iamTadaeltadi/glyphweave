//! Remapping for direct one-to-one and one-to-many substitutions.

use glyphweave_sfnt::{Decoded, GlyphId, NewGlyphId};

use crate::{AlternateSubstitution, MultipleSubstitution, SingleSubstitution};

pub(super) fn single(
    input: &SingleSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<SingleSubstitution<NewGlyphId>> {
    let mappings = input
        .mappings()
        .map(|(input, output)| {
            Ok((
                super::glyph(input, mapping)?,
                super::glyph(output, mapping)?,
            ))
        })
        .collect::<Decoded<Vec<_>>>()?;
    Ok(SingleSubstitution::from_mappings(mappings))
}

pub(super) fn multiple(
    input: &MultipleSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<MultipleSubstitution<NewGlyphId>> {
    let mappings = input
        .mappings()
        .map(|(input, outputs)| Ok((super::glyph(input, mapping)?, sequence(outputs, mapping)?)))
        .collect::<Decoded<Vec<_>>>()?;
    Ok(MultipleSubstitution::from_mappings(mappings))
}

pub(super) fn alternate(
    input: &AlternateSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<AlternateSubstitution<NewGlyphId>> {
    let mappings = input
        .mappings()
        .map(|(input, outputs)| Ok((super::glyph(input, mapping)?, sequence(outputs, mapping)?)))
        .collect::<Decoded<Vec<_>>>()?;
    Ok(AlternateSubstitution::from_alternates(mappings))
}

fn sequence(
    input: &[GlyphId],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<Vec<NewGlyphId>> {
    input
        .iter()
        .copied()
        .map(|input| super::glyph(input, mapping))
        .collect()
}

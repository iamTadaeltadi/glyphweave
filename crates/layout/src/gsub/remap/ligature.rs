//! Remapping for ordered ligature rules.

use glyphweave_sfnt::{Decoded, GlyphId, LayoutCause, NewGlyphId};

use crate::{LigatureRule, LigatureSubstitution, rewrite_error};

pub(super) fn substitution(
    input: &LigatureSubstitution,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<LigatureSubstitution<NewGlyphId>> {
    let rules = input
        .rules()
        .iter()
        .map(|rule| remap_rule(rule, mapping))
        .collect::<Decoded<Vec<_>>>()?;
    Ok(LigatureSubstitution::new(rules))
}

fn remap_rule(
    input: &LigatureRule,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Decoded<LigatureRule<NewGlyphId>> {
    let components = input
        .components()
        .iter()
        .copied()
        .map(|input| super::glyph(input, mapping))
        .collect::<Decoded<Vec<_>>>()?;
    let [first, second, remaining @ ..] = components.as_slice() else {
        let count = u16::try_from(components.len()).unwrap_or(u16::MAX);
        return Err(rewrite_error(LayoutCause::InvalidLigatureComponentCount {
            count,
        }));
    };
    let replacement = super::glyph(input.replacement(), mapping)?;
    Ok(LigatureRule::new(
        *first,
        *second,
        remaining.iter().copied(),
        replacement,
    ))
}

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use super::{direct, ligature, reverse};
use crate::{
    AlternateSubstitution, Coverage, LigatureRule, LigatureSubstitution, MultipleSubstitution,
    ReverseChainSingleSubstitution, SingleSubstitution,
};

#[test]
fn sequence_substitutions_preserve_output_order_and_deletion() {
    let multiple = MultipleSubstitution::from_mappings([
        (GlyphId::new(9), vec![GlyphId::new(12), GlyphId::new(4)]),
        (GlyphId::new(3), Vec::new()),
    ]);
    let alternate = AlternateSubstitution::from_alternates([(
        GlyphId::new(7),
        vec![GlyphId::new(10), GlyphId::new(8)],
    )]);
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 20));

    let multiple = direct::multiple(&multiple, &mut mapping).unwrap();
    let alternate = direct::alternate(&alternate, &mut mapping).unwrap();

    assert_eq!(multiple.replacement(NewGlyphId::new(23)), Some(&[][..]));
    assert_eq!(
        multiple.replacement(NewGlyphId::new(29)),
        Some(&[NewGlyphId::new(32), NewGlyphId::new(24)][..])
    );
    assert_eq!(
        alternate.alternates(NewGlyphId::new(27)),
        Some(&[NewGlyphId::new(30), NewGlyphId::new(28)][..])
    );
}

#[test]
fn ligature_and_reverse_context_order_survive_remapping() {
    let ligature = LigatureSubstitution::new([LigatureRule::new(
        GlyphId::new(2),
        GlyphId::new(3),
        [GlyphId::new(4)],
        GlyphId::new(9),
    )]);
    let reverse = ReverseChainSingleSubstitution::new(
        [Coverage::from_glyphs([GlyphId::new(8), GlyphId::new(1)])],
        SingleSubstitution::from_mappings([(GlyphId::new(3), GlyphId::new(7))]),
        [Coverage::from_glyphs([GlyphId::new(4)])],
    );
    let mut mapping = |glyph: GlyphId| Some(NewGlyphId::new(glyph.raw() + 10));

    let ligature = ligature::substitution(&ligature, &mut mapping).unwrap();
    let reverse = reverse::substitution(&reverse, &mut mapping).unwrap();

    assert_eq!(
        ligature.rules()[0].components(),
        [
            NewGlyphId::new(12),
            NewGlyphId::new(13),
            NewGlyphId::new(14)
        ]
    );
    assert_eq!(ligature.rules()[0].replacement(), NewGlyphId::new(19));
    assert_eq!(
        reverse.backtrack()[0].glyphs(),
        [NewGlyphId::new(11), NewGlyphId::new(18)]
    );
    assert_eq!(
        reverse.substitutions().replacement(NewGlyphId::new(13)),
        Some(NewGlyphId::new(17))
    );
    assert_eq!(reverse.lookahead()[0].glyphs(), [NewGlyphId::new(14)]);
}

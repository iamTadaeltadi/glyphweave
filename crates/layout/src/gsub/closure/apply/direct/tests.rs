use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::apply;
use crate::{
    AlternateSubstitution, Coverage, GsubSubtable, ReverseChainSingleSubstitution,
    SingleSubstitution,
};

#[test]
fn alternate_closure_adds_every_choice_for_a_reachable_input() {
    let subtable = GsubSubtable::Alternate(AlternateSubstitution::from_alternates([
        (GlyphId::new(3), vec![GlyphId::new(5), GlyphId::new(4)]),
        (GlyphId::new(8), vec![GlyphId::new(9)]),
    ]));
    let mut glyphs = [GlyphId::new(3)].into_iter().collect::<BTreeSet<_>>();

    apply(&subtable, &mut glyphs);

    assert_eq!(
        glyphs.into_iter().collect::<Vec<_>>(),
        [GlyphId::new(3), GlyphId::new(4), GlyphId::new(5)]
    );
}

#[test]
fn reverse_closure_requires_both_context_directions() {
    let subtable = GsubSubtable::ReverseChainSingle(ReverseChainSingleSubstitution::new(
        [Coverage::from_glyphs([GlyphId::new(1)])],
        SingleSubstitution::from_mappings([(GlyphId::new(3), GlyphId::new(4))]),
        [Coverage::from_glyphs([GlyphId::new(2)])],
    ));
    let mut feasible = [GlyphId::new(1), GlyphId::new(2), GlyphId::new(3)]
        .into_iter()
        .collect::<BTreeSet<_>>();
    let mut missing_lookahead = [GlyphId::new(1), GlyphId::new(3)]
        .into_iter()
        .collect::<BTreeSet<_>>();

    apply(&subtable, &mut feasible);
    apply(&subtable, &mut missing_lookahead);

    assert!(feasible.contains(&GlyphId::new(4)));
    assert!(!missing_lookahead.contains(&GlyphId::new(4)));
}

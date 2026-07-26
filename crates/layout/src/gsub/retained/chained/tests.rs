use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::retain;
use crate::{ChainedContextSubstitution, ContextRule, Coverage, GsubClosure, SubstitutionRecord};

#[test]
fn coverage_rule_retention_narrows_every_sequence_direction() {
    let input = ChainedContextSubstitution::CoverageRule(ContextRule::chained(
        [Coverage::from_glyphs([GlyphId::new(1), GlyphId::new(8)])],
        Coverage::from_glyphs([GlyphId::new(3), GlyphId::new(9)]),
        [],
        [Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(10)])],
        [SubstitutionRecord::new(0, 7)],
    ));

    let Some(ChainedContextSubstitution::CoverageRule(retained)) =
        retain(&input, &closed([1, 3, 4]))
    else {
        panic!("feasible chained Coverage rule must remain");
    };

    assert_eq!(retained.backtrack()[0].glyphs(), [GlyphId::new(1)]);
    assert_eq!(retained.input()[0].glyphs(), [GlyphId::new(3)]);
    assert_eq!(retained.lookahead()[0].glyphs(), [GlyphId::new(4)]);
    assert_eq!(retained.records(), [SubstitutionRecord::new(0, 7)]);
}

fn closed(raw_glyphs: impl IntoIterator<Item = u16>) -> GsubClosure {
    GsubClosure {
        glyphs: raw_glyphs
            .into_iter()
            .map(GlyphId::new)
            .collect::<BTreeSet<_>>(),
        lookups: BTreeSet::new(),
        passes: 0,
    }
}

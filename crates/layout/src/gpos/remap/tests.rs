//! Coverage, class, and per-glyph record rewriting for every GPOS subtable kind.

use std::collections::BTreeMap;

use glyphweave_sfnt::{ByteSpan, GlyphId, NewGlyphId};

use crate::{
    Anchor, BaseRecord, ChainedContextPositioning, ClassDefinition, ContextPositioning,
    ContextPositioningRecord, ContextPositioningRule, ContextRule, Coverage, CursivePositioning,
    EntryExit, GposLookupList, GposSubtable, MarkRecord, MarkToBasePositioning, PairAdjustment,
    PairPositioning, PairValue, SinglePositioning, SubstitutionRecord, ValueFormat, ValueRecord,
};

/// Builds a dense old-to-new remap from explicit pairs; unlisted glyphs are absent.
fn mapper(pairs: &[(u16, u16)]) -> impl FnMut(GlyphId) -> Option<NewGlyphId> {
    let map: BTreeMap<u16, u16> = pairs.iter().copied().collect();
    move |glyph: GlyphId| map.get(&glyph.raw()).copied().map(NewGlyphId::new)
}

const SINGLE_LOOKUP_LIST: [u8; 26] = [
    0x00, 0x01, // lookupCount
    0x00, 0x04, // lookupOffset[0]
    0x00, 0x01, // lookupType: single positioning
    0x00, 0x00, // lookupFlag
    0x00, 0x01, // subtableCount
    0x00, 0x08, // subtableOffset[0]
    0x00, 0x01, // SinglePos posFormat 1
    0x00, 0x08, // coverageOffset
    0x00, 0x04, // valueFormat: x advance
    0xff, 0xec, // shared valueRecord.xAdvance = -20
    0x00, 0x01, // Coverage format 1
    0x00, 0x01, // glyph count
    0x00, 0x05, // glyph id 5
];

#[test]
fn single_positioning_remaps_and_drops_absent_glyphs() {
    let input = SinglePositioning::new(
        ValueFormat::from_bits(0x0004).unwrap(),
        [
            (GlyphId::new(5), ValueRecord::new(0, 0, -20, 0)),
            (GlyphId::new(6), ValueRecord::new(0, 0, -30, 0)),
        ],
    );
    let mut map = mapper(&[(5, 2)]);
    let out = super::single::positioning(&input, &mut map);
    assert_eq!(out.len(), 1);
    assert_eq!(
        out.adjustment(NewGlyphId::new(2)),
        Some(ValueRecord::new(0, 0, -20, 0))
    );
    assert!(out.adjustment(NewGlyphId::new(3)).is_none());
}

#[test]
fn pair_glyphs_drops_absent_leading_and_trailing_glyphs() {
    let adjustment = PairAdjustment::new(ValueRecord::new(0, 0, -15, 0), ValueRecord::default());
    let input = PairPositioning::Glyphs {
        coverage: Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(8)]),
        sets: vec![
            vec![
                PairValue::new(GlyphId::new(7), adjustment),
                PairValue::new(GlyphId::new(9), adjustment),
            ],
            vec![PairValue::new(GlyphId::new(7), adjustment)],
        ],
    };
    let mut map = mapper(&[(4, 1), (7, 3)]);
    let out = super::pair::positioning(&input, &mut map);
    assert_eq!(out.coverage().glyphs(), &[NewGlyphId::new(1)]);
    assert_eq!(
        out.adjustment(NewGlyphId::new(1), NewGlyphId::new(3)),
        Some(adjustment)
    );
    assert!(
        out.adjustment(NewGlyphId::new(1), NewGlyphId::new(9))
            .is_none()
    );
}

#[test]
fn pair_classes_remaps_coverage_and_class_definitions() {
    let cell = PairAdjustment::new(ValueRecord::new(0, 0, -25, 0), ValueRecord::default());
    let input = PairPositioning::Classes {
        coverage: Coverage::from_glyphs([GlyphId::new(4)]),
        first_classes: ClassDefinition::from_assignments([(GlyphId::new(4), 1)]),
        second_classes: ClassDefinition::from_assignments([(GlyphId::new(7), 1)]),
        grid: vec![
            vec![PairAdjustment::default(), PairAdjustment::default()],
            vec![PairAdjustment::default(), cell],
        ],
    };
    let mut map = mapper(&[(4, 1), (7, 3)]);
    let out = super::pair::positioning(&input, &mut map);
    assert_eq!(
        out.adjustment(NewGlyphId::new(1), NewGlyphId::new(3)),
        Some(cell)
    );
}

#[test]
fn cursive_records_stay_aligned_when_the_remap_reorders_glyphs() {
    let entry3 = EntryExit::new(None, Some(Anchor::coordinate(70, 0)));
    let entry7 = EntryExit::new(Some(Anchor::coordinate(50, 0)), None);
    let input = CursivePositioning::new(
        Coverage::from_glyphs([GlyphId::new(3), GlyphId::new(7)]),
        [entry3, entry7],
    );
    let mut map = mapper(&[(3, 5), (7, 1)]);
    let out = super::cursive::positioning(&input, &mut map);
    assert_eq!(out.entry_exit(NewGlyphId::new(1)), Some(entry7));
    assert_eq!(out.entry_exit(NewGlyphId::new(5)), Some(entry3));
}

#[test]
fn mark_to_base_remaps_both_coverages_and_drops_absent_marks() {
    let input = MarkToBasePositioning::new(
        Coverage::from_glyphs([GlyphId::new(10), GlyphId::new(11)]),
        Coverage::from_glyphs([GlyphId::new(20)]),
        2,
        [
            MarkRecord::new(0, Anchor::coordinate(30, 0)),
            MarkRecord::new(1, Anchor::coordinate(-50, 10)),
        ],
        [BaseRecord::new([Some(Anchor::coordinate(200, 300)), None])],
    );
    let mut map = mapper(&[(10, 1), (20, 4)]);
    let out = super::mark::to_base(&input, &mut map);
    assert_eq!(out.mark_class_count(), 2);
    let attachment = out
        .attachment(NewGlyphId::new(1), NewGlyphId::new(4))
        .unwrap();
    assert_eq!(attachment.mark_class(), 0);
    assert_eq!(attachment.base_anchor(), Some(Anchor::coordinate(200, 300)));
    assert!(
        out.attachment(NewGlyphId::new(2), NewGlyphId::new(4))
            .is_none()
    );
}

#[test]
fn context_glyph_rule_is_dropped_when_a_predicate_glyph_is_absent() {
    let kept = ContextPositioningRule::new(
        GlyphId::new(4),
        [GlyphId::new(5)],
        [ContextPositioningRecord::new(0, 2)],
    );
    let dropped = ContextPositioningRule::new(
        GlyphId::new(4),
        [GlyphId::new(9)],
        [ContextPositioningRecord::new(0, 2)],
    );
    let input = ContextPositioning::GlyphRules(vec![kept, dropped]);
    let mut map = mapper(&[(4, 1), (5, 3)]);
    let ContextPositioning::GlyphRules(rules) = super::context::positioning(&input, &mut map)
    else {
        panic!("glyph rules preserved");
    };
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].input(), [NewGlyphId::new(1), NewGlyphId::new(3)]);
    assert_eq!(rules[0].records(), [ContextPositioningRecord::new(0, 2)]);
}

#[test]
fn chained_coverage_rule_remaps_and_empties_fully_dropped_positions() {
    let rule = ContextRule::chained(
        [Coverage::from_glyphs([GlyphId::new(2)])],
        Coverage::from_glyphs([GlyphId::new(3), GlyphId::new(4)]),
        [],
        [Coverage::from_glyphs([GlyphId::new(9)])],
        [SubstitutionRecord::new(0, 7)],
    );
    let input = ChainedContextPositioning::CoverageRule(rule);
    let mut map = mapper(&[(2, 1), (3, 5)]);
    let ChainedContextPositioning::CoverageRule(out) =
        super::chained::positioning(&input, &mut map)
    else {
        panic!("coverage rule preserved");
    };
    assert_eq!(out.backtrack()[0].glyphs(), &[NewGlyphId::new(1)]);
    assert_eq!(out.input()[0].glyphs(), &[NewGlyphId::new(5)]);
    assert!(out.lookahead()[0].is_empty());
    assert_eq!(out.records(), [SubstitutionRecord::new(0, 7)]);
}

#[test]
fn lookup_slots_survive_end_to_end_even_when_every_glyph_is_dropped() {
    let span = ByteSpan::sized(0, SINGLE_LOOKUP_LIST.len()).unwrap();
    let list = GposLookupList::decode(&SINGLE_LOOKUP_LIST, span).unwrap();

    let kept = list.remap(mapper(&[(5, 2)]));
    assert_eq!(kept.len(), 1);
    let GposSubtable::Single(single) = &kept.lookup(0).unwrap().subtables()[0] else {
        panic!("single positioning");
    };
    assert!(single.adjustment(NewGlyphId::new(2)).is_some());

    let dropped = list.remap(mapper(&[]));
    assert_eq!(dropped.len(), 1);
    let GposSubtable::Single(empty) = &dropped.lookup(0).unwrap().subtables()[0] else {
        panic!("single positioning");
    };
    assert!(empty.is_empty());
}

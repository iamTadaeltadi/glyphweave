//! Public determinism checks across M1 selection models.

use glyphweave_sfnt::{CharacterMap, FaceIndex, GlyphId, Tag};
use glyphweave_subset::{CodepointSet, Retention, SubsetPlan};

#[test]
fn equivalent_inputs_produce_identical_initial_plans() {
    let first_codepoints: CodepointSet = ['🦀', 'B', 'A'].into_iter().collect();
    let second_codepoints: CodepointSet = ['A', 'B', '🦀', 'A'].into_iter().collect();
    let first_map: CharacterMap = [
        ('🦀', GlyphId::new(70)),
        ('A', GlyphId::new(8)),
        ('B', GlyphId::new(12)),
    ]
    .into_iter()
    .collect();
    let second_map: CharacterMap = [
        ('B', GlyphId::new(12)),
        ('A', GlyphId::new(8)),
        ('🦀', GlyphId::new(70)),
    ]
    .into_iter()
    .collect();
    let cmap = Tag::new(*b"cmap");
    let glyf = Tag::new(*b"glyf");

    let first = SubsetPlan::from_character_map(
        FaceIndex::FIRST,
        first_codepoints,
        &first_map,
        [(glyf, Retention::Rewrite), (cmap, Retention::Rewrite)],
    );
    let second = SubsetPlan::from_character_map(
        FaceIndex::FIRST,
        second_codepoints,
        &second_map,
        [(cmap, Retention::Rewrite), (glyf, Retention::Rewrite)],
    );

    assert_eq!(first, second);
    assert_eq!(
        first
            .remap()
            .iter()
            .map(|(old, new)| (old.raw(), new.raw()))
            .collect::<Vec<_>>(),
        [(0, 0), (8, 1), (12, 2), (70, 3)]
    );
}

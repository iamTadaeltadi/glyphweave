use super::CharacterMap;
use crate::GlyphId;

#[test]
fn empty_map_resolves_every_scalar_to_notdef() {
    let map = CharacterMap::new();
    assert_eq!(map, CharacterMap::default());
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
    for codepoint in ['\0', 'A', '\u{10ffff}'] {
        assert_eq!(map.glyph_id(codepoint), GlyphId::NOTDEF);
    }
}

#[test]
fn empty_mapping_iteration_is_exact_and_double_ended() {
    let map = CharacterMap::new();
    let mut mappings = map.iter();
    assert_eq!(mappings.len(), 0);
    assert_eq!(mappings.next_back(), None);
    assert_eq!(mappings.next(), None);
}

#[test]
fn insertion_replaces_and_removes_explicit_mappings() {
    let mut map = CharacterMap::new();
    assert_eq!(map.insert('A', GlyphId::new(7)), None);
    assert_eq!(map.glyph_id('A'), GlyphId::new(7));
    assert_eq!(map.insert('A', GlyphId::new(9)), Some(GlyphId::new(7)));
    assert_eq!(map.remove('A'), Some(GlyphId::new(9)));
    assert_eq!(map.remove('A'), None);
    assert_eq!(map.glyph_id('A'), GlyphId::NOTDEF);
}

#[test]
fn inserting_notdef_canonicalizes_to_an_absent_mapping() {
    let mut map = CharacterMap::new();
    map.insert('A', GlyphId::new(7));
    assert_eq!(map.insert('A', GlyphId::NOTDEF), Some(GlyphId::new(7)));
    assert!(map.is_empty());
    assert_eq!(map.glyph_id('A'), GlyphId::NOTDEF);
}

#[test]
fn collection_canonicalizes_and_sorts_mappings() {
    let map: CharacterMap = [
        ('🦀', GlyphId::new(8)),
        ('A', GlyphId::new(2)),
        ('中', GlyphId::NOTDEF),
        ('A', GlyphId::new(3)),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        map.iter().collect::<Vec<_>>(),
        [('A', GlyphId::new(3)), ('🦀', GlyphId::new(8))]
    );
}

#[test]
fn retention_filters_by_scalar_and_glyph() {
    let mut map: CharacterMap = [
        ('A', GlyphId::new(1)),
        ('B', GlyphId::new(2)),
        ('β', GlyphId::new(3)),
    ]
    .into_iter()
    .collect();
    map.retain(|codepoint, glyph| codepoint.is_ascii() && glyph.raw() % 2 == 0);
    assert_eq!(map.iter().collect::<Vec<_>>(), [('B', GlyphId::new(2))]);
}

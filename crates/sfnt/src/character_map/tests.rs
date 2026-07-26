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

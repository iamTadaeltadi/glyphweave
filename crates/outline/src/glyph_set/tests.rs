use super::GlyphSet;
use glyphweave_sfnt::GlyphId;

#[test]
fn smallest_set_contains_only_notdef() {
    let glyphs = GlyphSet::new();
    assert_eq!(glyphs.len(), 1);
    assert!(!glyphs.is_empty());
    assert!(glyphs.contains(GlyphId::NOTDEF));
    assert_eq!(glyphs.iter().collect::<Vec<_>>(), [GlyphId::NOTDEF]);
}

#[test]
fn borrowed_iteration_is_exact_and_double_ended() {
    let glyphs = GlyphSet::default();
    let mut iter = (&glyphs).into_iter();
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next_back(), Some(GlyphId::NOTDEF));
    assert_eq!(iter.next(), None);
}

#[test]
fn insertion_reports_new_membership() {
    let mut glyphs = GlyphSet::new();
    assert!(glyphs.insert(GlyphId::new(9)));
    assert!(!glyphs.insert(GlyphId::new(9)));
    assert!(!glyphs.insert(GlyphId::NOTDEF));
    assert!(glyphs.contains(GlyphId::new(9)));
    assert_eq!(glyphs.len(), 2);
}

#[test]
fn removal_preserves_notdef() {
    let mut glyphs = GlyphSet::new();
    glyphs.insert(GlyphId::new(9));
    assert!(glyphs.remove(GlyphId::new(9)));
    assert!(!glyphs.remove(GlyphId::new(9)));
    assert!(!glyphs.remove(GlyphId::NOTDEF));
    assert_eq!(glyphs, GlyphSet::new());
}

#[test]
fn collection_deduplicates_and_sorts_glyphs() {
    let glyphs: GlyphSet = [7, 2, 7, 1].into_iter().map(GlyphId::new).collect();
    assert_eq!(
        glyphs.iter().map(GlyphId::raw).collect::<Vec<_>>(),
        [0, 1, 2, 7]
    );
}

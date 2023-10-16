use super::Coverage;
use glyphweave_sfnt::GlyphId;

#[test]
fn default_coverage_is_empty() {
    let coverage: Coverage = Coverage::default();
    assert!(coverage.is_empty());
    assert_eq!(coverage.len(), 0);
    assert!(coverage.glyphs().is_empty());
}

#[test]
fn construction_sorts_and_deduplicates_glyphs() {
    let coverage = Coverage::from_glyphs([
        GlyphId::new(7),
        GlyphId::new(2),
        GlyphId::new(7),
        GlyphId::NOTDEF,
    ]);
    assert_eq!(
        coverage.glyphs(),
        &[GlyphId::NOTDEF, GlyphId::new(2), GlyphId::new(7)]
    );
}

#[test]
fn lookup_is_bidirectional_for_present_glyphs() {
    let coverage = Coverage::from_glyphs([GlyphId::new(4), GlyphId::new(9)]);
    assert_eq!(coverage.coverage_index(GlyphId::new(4)), Some(0));
    assert_eq!(coverage.coverage_index(GlyphId::new(9)), Some(1));
    assert_eq!(coverage.glyph_at(0), Some(GlyphId::new(4)));
    assert_eq!(coverage.glyph_at(1), Some(GlyphId::new(9)));
}

#[test]
fn lookup_rejects_absent_glyphs_and_indices() {
    let coverage = Coverage::from_glyphs([GlyphId::new(4)]);
    assert_eq!(coverage.coverage_index(GlyphId::new(3)), None);
    assert_eq!(coverage.glyph_at(1), None);
}

#[test]
fn iterator_collection_preserves_canonical_form() {
    let coverage: Coverage = [3, 1, 3].into_iter().map(GlyphId::new).collect();
    assert_eq!(coverage.glyphs(), &[GlyphId::new(1), GlyphId::new(3)]);
}

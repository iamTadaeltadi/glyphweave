use glyphweave_outline::GlyphSet;
use glyphweave_sfnt::{FaceIndex, GlyphId, Tag};

use super::SubsetPlan;
use crate::{CodepointSet, Retention};

#[test]
fn plan_owns_selections_and_derives_their_remap() {
    let codepoints: CodepointSet = ['A', '🦀'].into_iter().collect();
    let glyphs: GlyphSet = [GlyphId::new(9), GlyphId::new(3)].into_iter().collect();
    let plan = SubsetPlan::new(FaceIndex::new(2), codepoints.clone(), glyphs.clone(), []);

    assert_eq!(plan.face_index(), FaceIndex::new(2));
    assert_eq!(plan.codepoints(), &codepoints);
    assert_eq!(plan.glyphs(), &glyphs);
    for glyph in &glyphs {
        let new_glyph = plan.remap().new_glyph_id(glyph).unwrap();
        assert_eq!(plan.remap().glyph_id(new_glyph), Some(glyph));
    }
}

#[test]
fn table_decisions_are_canonical_sorted_and_default_drop() {
    let cmap = Tag::new(*b"cmap");
    let glyf = Tag::new(*b"glyf");
    let head = Tag::new(*b"head");
    let plan = SubsetPlan::new(
        FaceIndex::FIRST,
        CodepointSet::new(),
        GlyphSet::new(),
        [
            (head, Retention::Keep),
            (glyf, Retention::Rewrite),
            (head, Retention::Drop),
        ],
    );

    assert_eq!(plan.retention(cmap), Retention::Drop);
    assert_eq!(plan.retention(head), Retention::Drop);
    assert_eq!(plan.retention(glyf), Retention::Rewrite);
    assert_eq!(
        plan.retentions().collect::<Vec<_>>(),
        [(glyf, Retention::Rewrite)]
    );
}

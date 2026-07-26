use crate::GlyphId;

use super::OutlineCause;

#[test]
fn formats_an_out_of_range_closure_root() {
    let cause = OutlineCause::GlyphClosureRootOutOfBounds {
        glyph: GlyphId::new(7),
        glyph_count: 4,
    };

    assert_eq!(
        cause.to_string(),
        "closure root glyph 7 is outside 4 available glyph(s)"
    );
}

#[test]
fn formats_a_dangling_component_with_its_parent() {
    let cause = OutlineCause::CompositeComponentOutOfBounds {
        parent: GlyphId::new(3),
        component: GlyphId::new(9),
        glyph_count: 5,
    };

    assert_eq!(
        cause.to_string(),
        "composite glyph 3 references glyph 9 outside 5 available glyph(s)"
    );
}

#[test]
fn formats_a_cycle_back_edge() {
    let cause = OutlineCause::CompositeGlyphCycle {
        parent: GlyphId::new(8),
        component: GlyphId::new(2),
    };

    assert_eq!(
        cause.to_string(),
        "composite glyph 8 references active ancestor glyph 2"
    );
}

//! Live semantic comparisons with FontTools when it is installed.

mod support;

use glyphweave_outline::{CompositeGlyph, SimpleGlyph};

use support::{reference_font::glyf, reference_tool::ttx_xml};

#[test]
fn ttx_matches_the_glyphweave_outline_census() {
    let Some(xml) = ttx_xml(&["GlyphOrder", "glyf"]) else {
        return;
    };
    let glyf = glyf();
    let simple_contours: usize = glyf
        .outlines()
        .iter()
        .filter_map(|outline| outline.as_simple())
        .map(SimpleGlyph::contour_count)
        .sum();
    let components: usize = glyf
        .outlines()
        .iter()
        .filter_map(|outline| outline.as_composite())
        .map(CompositeGlyph::component_count)
        .sum();

    assert_eq!(xml.matches("<GlyphID ").count(), glyf.glyph_count());
    assert_eq!(xml.matches("<TTGlyph ").count(), glyf.glyph_count());
    assert_eq!(xml.matches("<contour>").count(), simple_contours);
    assert_eq!(xml.matches("<component ").count(), components);
}

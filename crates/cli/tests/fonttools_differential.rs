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

#[test]
fn ttx_matches_every_glyphweave_outline_bound() {
    let Some(xml) = ttx_xml(&["glyf"]) else {
        return;
    };
    let names = [".notdef", "space", "triangle", "acute", "Aacute", "nested"];

    for ((_, outline), name) in glyf().iter().zip(names) {
        if let Some(bounds) = outline.bounds() {
            let expected = format!(
                "<TTGlyph name=\"{name}\" xMin=\"{}\" yMin=\"{}\" xMax=\"{}\" yMax=\"{}\">",
                bounds.minimum_x(),
                bounds.minimum_y(),
                bounds.maximum_x(),
                bounds.maximum_y()
            );
            assert!(xml.contains(&expected), "ttx omitted {expected}");
        } else {
            let expected = format!("<TTGlyph name=\"{name}\"/>");
            assert!(xml.contains(&expected), "ttx omitted {expected}");
        }
    }
}

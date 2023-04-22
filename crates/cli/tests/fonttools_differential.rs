//! Live semantic comparisons with FontTools when it is installed.

mod support;

use std::collections::BTreeMap;

use glyphweave_outline::{CompositeGlyph, F2Dot14, SimpleGlyph};

use support::{
    reference_font::glyf,
    reference_map::{format4, format12, format14},
    reference_tool::ttx_xml,
};

const GLYPH_NAMES: [&str; 6] = [".notdef", "space", "triangle", "acute", "Aacute", "nested"];

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
    for ((_, outline), name) in glyf().iter().zip(GLYPH_NAMES) {
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

#[test]
fn ttx_matches_the_glyphweave_point_multiset() {
    let Some(xml) = ttx_xml(&["glyf"]) else {
        return;
    };
    let mut expected = BTreeMap::new();
    for glyph in glyf()
        .outlines()
        .iter()
        .filter_map(|outline| outline.as_simple())
    {
        for point in glyph.points() {
            *expected
                .entry((point.x(), point.y(), point.is_on_curve()))
                .or_insert(0usize) += 1;
        }
    }

    for ((x, y, on_curve), count) in &expected {
        let needle = format!("<pt x=\"{x}\" y=\"{y}\" on=\"{}\"/>", u8::from(*on_curve));
        assert_eq!(
            xml.matches(&needle).count(),
            *count,
            "ttx count for {needle}"
        );
    }
    assert_eq!(
        xml.matches("<pt ").count(),
        expected.values().sum::<usize>()
    );
}

#[test]
fn ttx_matches_the_scaled_nested_component() {
    let Some(xml) = ttx_xml(&["glyf"]) else {
        return;
    };
    let glyf = glyf();
    let component = &glyf
        .outline(glyphweave_sfnt::GlyphId::new(5))
        .unwrap()
        .as_composite()
        .unwrap()
        .components()
        .get(1)
        .unwrap();

    assert_eq!(component.glyph_id().raw(), 2);
    assert_eq!(
        component.transform().coefficients().map(F2Dot14::to_bits),
        [0x2000, 0, 0, 0x2000]
    );
    assert!(xml.contains(
        "<component glyphName=\"triangle\" x=\"300\" y=\"0\" scale=\"0.5\" flags=\"0x4\"/>"
    ));
}

#[test]
fn ttx_matches_the_glyphweave_format_four_map() {
    let Some(xml) = ttx_xml(&["cmap"]) else {
        return;
    };
    let section = xml_section(
        &xml,
        "<cmap_format_4 platformID=\"0\" platEncID=\"3\"",
        "</cmap_format_4>",
    );
    let mappings = format4();

    for (codepoint, glyph) in mappings.character_map().iter() {
        let name = GLYPH_NAMES.get(usize::from(glyph.raw())).unwrap();
        let expected = format!(
            "<map code=\"0x{:x}\" name=\"{name}\"/>",
            u32::from(codepoint)
        );
        assert!(section.contains(&expected), "ttx omitted {expected}");
    }
    assert_eq!(
        section.matches("<map ").count(),
        mappings.character_map().len()
    );
}

#[test]
fn ttx_matches_the_glyphweave_format_twelve_map() {
    let Some(xml) = ttx_xml(&["cmap"]) else {
        return;
    };
    let section = xml_section(
        &xml,
        "<cmap_format_12 platformID=\"3\" platEncID=\"10\"",
        "</cmap_format_12>",
    );
    let mappings = format12();

    for (codepoint, glyph) in mappings.character_map().iter() {
        let name = GLYPH_NAMES.get(usize::from(glyph.raw())).unwrap();
        let expected = format!(
            "<map code=\"0x{:x}\" name=\"{name}\"/>",
            u32::from(codepoint)
        );
        assert!(section.contains(&expected), "ttx omitted {expected}");
    }
    assert_eq!(
        section.matches("<map ").count(),
        mappings.character_map().len()
    );
}

#[test]
fn ttx_matches_the_glyphweave_variation_sequences() {
    let Some(xml) = ttx_xml(&["cmap"]) else {
        return;
    };
    let section = xml_section(
        &xml,
        "<cmap_format_14 platformID=\"0\" platEncID=\"5\">",
        "</cmap_format_14>",
    );
    let mappings = format14();
    let variation = mappings.variation_selector_map();
    let mut expected_count = 0;

    for (selector, range) in variation.default_ranges() {
        for base in range {
            let expected = format!(
                "<map uv=\"0x{:x}\" uvs=\"0x{:x}\"/>",
                u32::from(base),
                u32::from(selector.value())
            );
            assert!(section.contains(&expected), "ttx omitted {expected}");
            expected_count += 1;
        }
    }
    for (selector, base, glyph) in variation.non_default_mappings() {
        let name = GLYPH_NAMES.get(usize::from(glyph.raw())).unwrap();
        let expected = format!(
            "<map uv=\"0x{:x}\" uvs=\"0x{:x}\" name=\"{name}\"/>",
            u32::from(base),
            u32::from(selector.value())
        );
        assert!(section.contains(&expected), "ttx omitted {expected}");
        expected_count += 1;
    }
    assert_eq!(section.matches("<map ").count(), expected_count);
}

fn xml_section<'a>(xml: &'a str, opening: &str, closing: &str) -> &'a str {
    let suffix = xml.split_once(opening).expect("ttx opening element").1;
    suffix.split_once(closing).expect("ttx closing element").0
}

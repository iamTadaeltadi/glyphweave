//! Generated point-alignment properties for composite glyphs.

use glyphweave_outline::{
    Component, ComponentOffsetMode, ComponentPlacement, ComponentTransform, CompositeGlyph,
};
use glyphweave_sfnt::{ByteSpan, GlyphId};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x504f_494e),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_point_records_match_unsigned_reference_models(
        points in collection::vec(
            (any::<u16>(), any::<u16>(), any::<u16>(), any::<bool>(), any::<bool>(), 0u8..3),
            0..33,
        ),
        prefix in collection::vec(any::<u8>(), 0..17),
    ) {
        let mut encoded = vec![0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0];
        let first_flags = if points.is_empty() { 0x0002_u16 } else { 0x0022 };
        encoded.extend(first_flags.to_be_bytes());
        encoded.extend([0, 0, 0, 0]);
        let mut expected = vec![Component::new(
            GlyphId::NOTDEF,
            ComponentPlacement::Offset {
                x: 0,
                y: 0,
                round_to_grid: false,
                mode: ComponentOffsetMode::Default,
            },
            ComponentTransform::identity(),
            false,
        )];

        for (index, &(glyph_id, parent, child, words, metrics, ignored_mode))
            in points.iter().enumerate()
        {
            let mut flags = u16::from(words);
            flags |= if index + 1 < points.len() { 0x0020 } else { 0 };
            flags |= if metrics { 0x0200 } else { 0 };
            flags |= match ignored_mode {
                0 => 0,
                1 => 0x0004,
                _ => 0x0800,
            };
            encoded.extend(flags.to_be_bytes());
            encoded.extend(glyph_id.to_be_bytes());
            let (expected_parent, expected_child) = if words {
                encoded.extend(parent.to_be_bytes());
                encoded.extend(child.to_be_bytes());
                (parent, child)
            } else {
                let parent = parent.to_be_bytes()[1];
                let child = child.to_be_bytes()[1];
                encoded.extend([parent, child]);
                (u16::from(parent), u16::from(child))
            };
            expected.push(Component::new(
                GlyphId::new(glyph_id),
                ComponentPlacement::MatchPoints {
                    parent: expected_parent,
                    child: expected_child,
                },
                ComponentTransform::identity(),
                metrics,
            ));
        }

        let mut bytes = prefix;
        let start = bytes.len();
        bytes.extend(&encoded);
        let span = ByteSpan::sized(start, encoded.len()).unwrap();

        let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

        prop_assert_eq!(glyph.components(), expected.as_slice());
    }
}

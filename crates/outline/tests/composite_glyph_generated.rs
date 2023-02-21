//! Generated valid-table properties for composite glyph records.

use glyphweave_outline::{
    Component, ComponentOffsetMode, ComponentPlacement, ComponentTransform, CompositeGlyph, F2Dot14,
};
use glyphweave_sfnt::{ByteSpan, GlyphId};
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

#[derive(Clone, Copy, Debug)]
enum GeneratedTransform {
    Identity,
    Uniform(i16),
    Axis(i16, i16),
    Matrix(i16, i16, i16, i16),
}

fn transform_strategy() -> impl Strategy<Value = GeneratedTransform> {
    prop_oneof![
        Just(GeneratedTransform::Identity),
        any::<i16>().prop_map(GeneratedTransform::Uniform),
        (any::<i16>(), any::<i16>()).prop_map(|(x, y)| GeneratedTransform::Axis(x, y)),
        (any::<i16>(), any::<i16>(), any::<i16>(), any::<i16>())
            .prop_map(|(x, a, b, y)| GeneratedTransform::Matrix(x, a, b, y)),
    ]
}

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x4745_4e43),
        ..Config::default()
    }
}

fn encode_transform(
    transform: GeneratedTransform,
    bytes: &mut Vec<u8>,
) -> (u16, ComponentTransform) {
    match transform {
        GeneratedTransform::Identity => (0, ComponentTransform::identity()),
        GeneratedTransform::Uniform(scale) => {
            bytes.extend(scale.to_be_bytes());
            (
                0x0008,
                ComponentTransform::Uniform {
                    scale: F2Dot14::from_bits(scale),
                },
            )
        }
        GeneratedTransform::Axis(x_scale, y_scale) => {
            bytes.extend(x_scale.to_be_bytes());
            bytes.extend(y_scale.to_be_bytes());
            (
                0x0040,
                ComponentTransform::Axis {
                    x_scale: F2Dot14::from_bits(x_scale),
                    y_scale: F2Dot14::from_bits(y_scale),
                },
            )
        }
        GeneratedTransform::Matrix(x_scale, scale_01, scale_10, y_scale) => {
            for value in [x_scale, scale_01, scale_10, y_scale] {
                bytes.extend(value.to_be_bytes());
            }
            (
                0x0080,
                ComponentTransform::Matrix {
                    x_scale: F2Dot14::from_bits(x_scale),
                    scale_01: F2Dot14::from_bits(scale_01),
                    scale_10: F2Dot14::from_bits(scale_10),
                    y_scale: F2Dot14::from_bits(y_scale),
                },
            )
        }
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_components_match_their_canonical_models(
        values in collection::vec(
            (any::<u16>(), any::<i16>(), any::<i16>(), transform_strategy(), any::<bool>(), any::<bool>(), 0u8..3),
            1..33,
        ),
        instructions in collection::vec(any::<u8>(), 0..65),
        instruction_owner in any::<usize>(),
        overlap in any::<bool>(),
        prefix in collection::vec(any::<u8>(), 0..17),
    ) {
        let owner = instruction_owner % values.len();
        let mut encoded = vec![0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0];
        let mut expected = Vec::with_capacity(values.len());
        for (index, &(glyph, x, y, generated_transform, metrics, round, mode_value))
            in values.iter().enumerate()
        {
            let mut transform_bytes = Vec::new();
            let (transform_flag, transform) = encode_transform(generated_transform, &mut transform_bytes);
            let mode = match mode_value {
                0 => ComponentOffsetMode::Default,
                1 => ComponentOffsetMode::Scaled,
                _ => ComponentOffsetMode::Unscaled,
            };
            let mut flags = 0x0003 | transform_flag;
            flags |= if index + 1 < values.len() { 0x0020 } else { 0 };
            flags |= if metrics { 0x0200 } else { 0 };
            flags |= if round { 0x0004 } else { 0 };
            flags |= if index == 0 && overlap { 0x0400 } else { 0 };
            flags |= match mode {
                ComponentOffsetMode::Default => 0,
                ComponentOffsetMode::Scaled => 0x0800,
                ComponentOffsetMode::Unscaled => 0x1000,
            };
            flags |= if index == owner { 0x0100 } else { 0 };
            encoded.extend(flags.to_be_bytes());
            encoded.extend(glyph.to_be_bytes());
            encoded.extend(x.to_be_bytes());
            encoded.extend(y.to_be_bytes());
            encoded.extend(transform_bytes);
            expected.push(Component::new(
                GlyphId::new(glyph),
                ComponentPlacement::Offset { x, y, round_to_grid: round, mode },
                transform,
                metrics,
            ));
        }
        encoded.extend(u16::try_from(instructions.len()).unwrap().to_be_bytes());
        encoded.extend(&instructions);
        if encoded.len() % 2 != 0 {
            encoded.push(0);
        }
        let mut bytes = prefix;
        let start = bytes.len();
        bytes.extend(&encoded);
        let span = ByteSpan::sized(start, encoded.len()).unwrap();

        let glyph = CompositeGlyph::decode(&bytes, span).unwrap();

        prop_assert_eq!(glyph.components(), expected.as_slice());
        prop_assert_eq!(glyph.instructions(), instructions.as_slice());
        prop_assert_eq!(glyph.has_overlapping_components(), overlap);
    }
}

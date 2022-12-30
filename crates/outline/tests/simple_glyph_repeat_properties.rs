//! Generated flag-run properties for simple glyphs.

use glyphweave_outline::{Point, SimpleGlyph};
use glyphweave_sfnt::ByteSpan;
use proptest::{
    collection,
    prelude::*,
    test_runner::{Config, RngSeed},
};

fn fixed_config() -> Config {
    Config {
        cases: 512,
        failure_persistence: None,
        rng_seed: RngSeed::Fixed(0x5255_4e53),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_flag_runs_expand_to_their_logical_point_model(
        runs in collection::vec((any::<bool>(), 1u8..=32), 1..33),
        overlap in any::<bool>(),
    ) {
        let point_count: u32 = runs.iter().map(|(_, length)| u32::from(*length)).sum();
        let mut bytes = Vec::new();
        for value in [1i16, 0, 0, 0, 0] {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        bytes.extend_from_slice(&u16::try_from(point_count - 1).unwrap().to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes());
        let mut expected = Vec::with_capacity(usize::try_from(point_count).unwrap());
        for (index, &(on_curve, length)) in runs.iter().enumerate() {
            let mut flag = 0x30 | u8::from(on_curve);
            if index == 0 && overlap {
                flag |= 0x40;
            }
            if length > 1 {
                flag |= 0x08;
            }
            bytes.push(flag);
            if length > 1 {
                bytes.push(length - 1);
            }
            expected.extend(core::iter::repeat_n(
                Point::new(0, 0, on_curve),
                usize::from(length),
            ));
        }
        if bytes.len() % 2 != 0 {
            bytes.push(0);
        }
        let span = ByteSpan::sized(0, bytes.len()).unwrap();

        let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

        prop_assert_eq!(glyph.points(), expected.as_slice());
        prop_assert_eq!(glyph.contour_ends(), [point_count]);
        prop_assert_eq!(glyph.has_overlapping_contours(), overlap);
    }
}

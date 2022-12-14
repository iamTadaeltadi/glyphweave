//! Generated contour-partition properties for simple glyphs.

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
        rng_seed: RngSeed::Fixed(0x434f_4e54),
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_contour_lengths_partition_the_canonical_point_array(
        lengths in collection::vec(1u16..=16, 1..33),
    ) {
        let contour_count = i16::try_from(lengths.len()).unwrap();
        let mut bytes = Vec::new();
        for value in [contour_count, 0, 0, 0, 0] {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        let mut expected_ends = Vec::with_capacity(lengths.len());
        let mut point_count = 0u32;
        for length in &lengths {
            point_count += u32::from(*length);
            expected_ends.push(point_count);
            bytes.extend_from_slice(&u16::try_from(point_count - 1).unwrap().to_be_bytes());
        }
        bytes.extend_from_slice(&0u16.to_be_bytes());
        bytes.extend(core::iter::repeat_n(0x31, usize::try_from(point_count).unwrap()));
        if bytes.len() % 2 != 0 {
            bytes.push(0);
        }
        let span = ByteSpan::sized(0, bytes.len()).unwrap();

        let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

        prop_assert_eq!(glyph.contour_ends(), expected_ends.as_slice());
        prop_assert_eq!(glyph.points().len(), usize::try_from(point_count).unwrap());
        for (index, length) in lengths.iter().enumerate() {
            let contour = glyph.contour(index).unwrap();
            prop_assert_eq!(contour.len(), usize::from(*length));
            prop_assert!(contour.iter().all(|point| *point == Point::new(0, 0, true)));
        }
    }
}

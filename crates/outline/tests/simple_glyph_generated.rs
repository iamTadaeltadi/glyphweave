//! Generated valid-table properties for packed simple glyphs.

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
        rng_seed: RngSeed::Fixed(0x5349_4d50),
        ..Config::default()
    }
}

fn encode(points: &[(i8, i8, bool)], instructions: &[u8], overlap: bool) -> (Vec<u8>, Vec<Point>) {
    let mut expected = Vec::with_capacity(points.len());
    let mut flags = Vec::with_capacity(points.len());
    let mut x_bytes = Vec::new();
    let mut y_bytes = Vec::new();
    let mut x = 0i16;
    let mut y = 0i16;
    for (index, &(delta_x, delta_y, on_curve)) in points.iter().enumerate() {
        let mut flag = u8::from(on_curve);
        flag |= encode_short(delta_x, 0x02, 0x10, &mut x_bytes);
        flag |= encode_short(delta_y, 0x04, 0x20, &mut y_bytes);
        if index == 0 && overlap {
            flag |= 0x40;
        }
        flags.push(flag);
        x += i16::from(delta_x);
        y += i16::from(delta_y);
        expected.push(Point::new(x, y, on_curve));
    }

    let minimum_x = expected.iter().map(|point| point.x()).min().unwrap();
    let minimum_y = expected.iter().map(|point| point.y()).min().unwrap();
    let maximum_x = expected.iter().map(|point| point.x()).max().unwrap();
    let maximum_y = expected.iter().map(|point| point.y()).max().unwrap();
    let mut bytes = Vec::new();
    for value in [1, minimum_x, minimum_y, maximum_x, maximum_y] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    let end_point = u16::try_from(points.len() - 1).unwrap();
    bytes.extend_from_slice(&end_point.to_be_bytes());
    bytes.extend_from_slice(&u16::try_from(instructions.len()).unwrap().to_be_bytes());
    bytes.extend_from_slice(instructions);
    bytes.extend_from_slice(&flags);
    bytes.extend_from_slice(&x_bytes);
    bytes.extend_from_slice(&y_bytes);
    if bytes.len() % 2 != 0 {
        bytes.push(0);
    }
    (bytes, expected)
}

fn encode_short(delta: i8, short: u8, positive: u8, output: &mut Vec<u8>) -> u8 {
    if delta == 0 {
        return positive;
    }
    output.push(delta.unsigned_abs());
    short | if delta > 0 { positive } else { 0 }
}

proptest! {
    #![proptest_config(fixed_config())]

    #[test]
    fn generated_short_vectors_match_absolute_point_models(
        deltas in collection::vec((-120i8..=120, -120i8..=120, any::<bool>()), 1..65),
        instructions in collection::vec(any::<u8>(), 0..65),
        overlap in any::<bool>(),
        prefix in collection::vec(any::<u8>(), 0..17),
    ) {
        let (encoded, expected) = encode(&deltas, &instructions, overlap);
        let mut bytes = prefix;
        let start = bytes.len();
        bytes.extend_from_slice(&encoded);
        let span = ByteSpan::sized(start, encoded.len()).unwrap();

        let glyph = SimpleGlyph::decode(&bytes, span).unwrap();

        prop_assert_eq!(glyph.points(), expected.as_slice());
        prop_assert_eq!(glyph.contour_ends(), [u32::try_from(expected.len()).unwrap()]);
        prop_assert_eq!(glyph.instructions(), instructions.as_slice());
        prop_assert_eq!(glyph.has_overlapping_contours(), overlap);
    }
}

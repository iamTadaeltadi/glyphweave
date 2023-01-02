use super::F2Dot14;

#[test]
fn preserves_every_fixed_point_bit_pattern() {
    for bits in i16::MIN..=i16::MAX {
        assert_eq!(F2Dot14::from_bits(bits).to_bits(), bits);
    }
}

#[test]
fn converts_exact_landmark_values() {
    assert_eq!(F2Dot14::ZERO.to_f64(), 0.0);
    assert_eq!(F2Dot14::ONE.to_f64(), 1.0);
    assert_eq!(F2Dot14::from_bits(i16::MIN).to_f64(), -2.0);
    assert_eq!(F2Dot14::from_bits(i16::MAX).to_f64(), 1.999_938_964_843_75);
}

use super::ComponentTransform;
use crate::F2Dot14;

#[test]
fn compact_transforms_expand_to_complete_matrices() {
    let half = F2Dot14::from_bits(0x2000);
    let quarter = F2Dot14::from_bits(0x1000);

    assert_eq!(
        ComponentTransform::identity().coefficients(),
        [F2Dot14::ONE, F2Dot14::ZERO, F2Dot14::ZERO, F2Dot14::ONE]
    );
    assert_eq!(
        ComponentTransform::Uniform { scale: half }.coefficients(),
        [half, F2Dot14::ZERO, F2Dot14::ZERO, half]
    );
    assert_eq!(
        ComponentTransform::Axis {
            x_scale: half,
            y_scale: quarter,
        }
        .coefficients(),
        [half, F2Dot14::ZERO, F2Dot14::ZERO, quarter]
    );
    assert_eq!(
        ComponentTransform::Matrix {
            x_scale: half,
            scale_01: quarter,
            scale_10: F2Dot14::from_bits(-0x1000),
            y_scale: half,
        }
        .coefficients(),
        [half, quarter, F2Dot14::from_bits(-0x1000), half]
    );
}

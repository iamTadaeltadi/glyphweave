//! Decoding of exact component transform coefficients.

use glyphweave_sfnt::{Decoded, Reader};

use super::flags::ComponentFlags;
use crate::{ComponentTransform, F2Dot14};

pub(super) fn decode(
    reader: &mut Reader<'_>,
    flags: ComponentFlags,
) -> Decoded<ComponentTransform> {
    if flags.has_scale() {
        return Ok(ComponentTransform::Uniform {
            scale: coefficient(reader)?,
        });
    }
    if flags.has_axis_scales() {
        return Ok(ComponentTransform::Axis {
            x_scale: coefficient(reader)?,
            y_scale: coefficient(reader)?,
        });
    }
    if flags.has_matrix() {
        return Ok(ComponentTransform::Matrix {
            x_scale: coefficient(reader)?,
            scale_01: coefficient(reader)?,
            scale_10: coefficient(reader)?,
            y_scale: coefficient(reader)?,
        });
    }
    Ok(ComponentTransform::identity())
}

fn coefficient(reader: &mut Reader<'_>) -> Decoded<F2Dot14> {
    Ok(F2Dot14::from_bits(reader.i16()?))
}

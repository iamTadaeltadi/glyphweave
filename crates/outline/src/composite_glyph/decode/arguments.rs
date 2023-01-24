//! Canonicalization of variable-width component placement arguments.

use glyphweave_sfnt::{Decoded, Reader};

use super::flags::ComponentFlags;
use crate::{ComponentOffsetMode, ComponentPlacement};

pub(super) fn decode(
    reader: &mut Reader<'_>,
    flags: ComponentFlags,
) -> Decoded<ComponentPlacement> {
    if flags.arguments_are_xy_values() {
        let (x, y) = decode_signed_pair(reader, flags.arguments_are_words())?;
        return Ok(ComponentPlacement::Offset {
            x,
            y,
            round_to_grid: flags.round_to_grid(),
            mode: offset_mode(flags),
        });
    }
    let (parent, child) = decode_unsigned_pair(reader, flags.arguments_are_words())?;
    Ok(ComponentPlacement::MatchPoints { parent, child })
}

fn decode_signed_pair(reader: &mut Reader<'_>, words: bool) -> Decoded<(i16, i16)> {
    if words {
        Ok((reader.i16()?, reader.i16()?))
    } else {
        Ok((signed_byte(reader.u8()?), signed_byte(reader.u8()?)))
    }
}

fn decode_unsigned_pair(reader: &mut Reader<'_>, words: bool) -> Decoded<(u16, u16)> {
    if words {
        Ok((reader.u16()?, reader.u16()?))
    } else {
        Ok((u16::from(reader.u8()?), u16::from(reader.u8()?)))
    }
}

const fn signed_byte(value: u8) -> i16 {
    i8::from_be_bytes([value]) as i16
}

const fn offset_mode(flags: ComponentFlags) -> ComponentOffsetMode {
    if flags.uses_scaled_offset() {
        ComponentOffsetMode::Scaled
    } else if flags.uses_unscaled_offset() {
        ComponentOffsetMode::Unscaled
    } else {
        ComponentOffsetMode::Default
    }
}

//! Exact expansion of packed simple-glyph flag runs.

use glyphweave_sfnt::{ByteSpan, Cause, Decoded, Diagnostic, OutlineCause, Reader, Stage};

const ON_CURVE_POINT: u8 = 0x01;
pub(super) const X_SHORT_VECTOR: u8 = 0x02;
pub(super) const Y_SHORT_VECTOR: u8 = 0x04;
const REPEAT_FLAG: u8 = 0x08;
pub(super) const X_SAME_OR_POSITIVE: u8 = 0x10;
pub(super) const Y_SAME_OR_POSITIVE: u8 = 0x20;
const OVERLAP_SIMPLE: u8 = 0x40;
const RESERVED: u8 = 0x80;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct PointFlag(u8);

impl PointFlag {
    pub(super) const fn is_on_curve(self) -> bool {
        self.0 & ON_CURVE_POINT != 0
    }

    pub(super) const fn contains(self, mask: u8) -> bool {
        self.0 & mask != 0
    }
}

pub(super) struct ExpandedFlags {
    values: Vec<PointFlag>,
    overlapping_contours: bool,
}

impl ExpandedFlags {
    pub(super) fn len(&self) -> usize {
        self.values.len()
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = PointFlag> + '_ {
        self.values.iter().copied()
    }

    pub(super) const fn overlapping_contours(&self) -> bool {
        self.overlapping_contours
    }
}

pub(super) fn decode(reader: &mut Reader<'_>, point_count: u32) -> Decoded<ExpandedFlags> {
    let capacity = usize::try_from(point_count).unwrap_or(usize::MAX);
    let mut values = Vec::with_capacity(capacity);
    let mut overlapping_contours = false;

    while values.len() < capacity {
        let flag_offset = reader.offset();
        let raw = reader.u8()?;
        let point_index = u32::try_from(values.len()).unwrap_or(u32::MAX);
        if raw & RESERVED != 0 {
            return Err(flag_diagnostic(
                flag_offset,
                OutlineCause::ReservedSimpleGlyphFlag {
                    point_index,
                    flag: raw,
                },
            ));
        }
        if raw & OVERLAP_SIMPLE != 0 {
            if values.is_empty() {
                overlapping_contours = true;
            } else {
                return Err(flag_diagnostic(
                    flag_offset,
                    OutlineCause::MisplacedSimpleOverlapFlag { point_index },
                ));
            }
        }

        let additional_repetitions = if raw & REPEAT_FLAG == 0 {
            0
        } else {
            reader.u8()?
        };
        let run_length = usize::from(additional_repetitions) + 1;
        if run_length > capacity - values.len() {
            let repeat_offset = reader.offset().saturating_sub(1);
            return Err(flag_diagnostic(
                repeat_offset,
                OutlineCause::SimpleFlagRepeatOutOfBounds {
                    point_index,
                    additional_repetitions,
                    point_count,
                },
            ));
        }
        let logical = PointFlag(raw & !(REPEAT_FLAG | OVERLAP_SIMPLE));
        values.extend(core::iter::repeat_n(logical, run_length));
    }

    Ok(ExpandedFlags {
        values,
        overlapping_contours,
    })
}

fn flag_diagnostic(offset: usize, cause: OutlineCause) -> Diagnostic {
    Diagnostic::at(
        Stage::Decode,
        ByteSpan::sized(offset, 1).unwrap_or_else(|| ByteSpan::empty_at(offset)),
        Cause::Outline(cause),
    )
}

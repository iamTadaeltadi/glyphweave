//! Validation and interpretation of composite component flag words.

use glyphweave_sfnt::{ByteSpan, Cause, Decoded, Diagnostic, OutlineCause, Reader, Stage};

const ARGUMENTS_ARE_WORDS: u16 = 0x0001;
const ARGUMENTS_ARE_XY_VALUES: u16 = 0x0002;
const ROUND_XY_TO_GRID: u16 = 0x0004;
const HAVE_SCALE: u16 = 0x0008;
const MORE_COMPONENTS: u16 = 0x0020;
const HAVE_AXIS_SCALES: u16 = 0x0040;
const HAVE_MATRIX: u16 = 0x0080;
const HAVE_INSTRUCTIONS: u16 = 0x0100;
const USE_MY_METRICS: u16 = 0x0200;
const OVERLAP_COMPOUND: u16 = 0x0400;
const SCALED_COMPONENT_OFFSET: u16 = 0x0800;
const UNSCALED_COMPONENT_OFFSET: u16 = 0x1000;
const RESERVED: u16 = 0xe010;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ComponentFlags(u16);

impl ComponentFlags {
    pub(super) fn decode(reader: &mut Reader<'_>, component_index: u32) -> Decoded<Self> {
        let offset = reader.offset();
        let flags = Self(reader.u16()?);
        let span = ByteSpan::sized(offset, 2).unwrap_or_else(|| ByteSpan::empty_at(offset));
        flags.validate(component_index, span)?;
        Ok(flags)
    }

    fn validate(self, component_index: u32, span: ByteSpan) -> Decoded<()> {
        if self.0 & RESERVED != 0 {
            return Err(diagnostic(
                span,
                OutlineCause::ReservedCompositeGlyphFlags {
                    component_index,
                    flags: self.0,
                },
            ));
        }
        let transform_count = [HAVE_SCALE, HAVE_AXIS_SCALES, HAVE_MATRIX]
            .into_iter()
            .filter(|mask| self.contains(*mask))
            .count();
        if transform_count > 1 {
            return Err(diagnostic(
                span,
                OutlineCause::ConflictingCompositeTransforms {
                    component_index,
                    flags: self.0,
                },
            ));
        }
        if self.contains(SCALED_COMPONENT_OFFSET) && self.contains(UNSCALED_COMPONENT_OFFSET) {
            return Err(diagnostic(
                span,
                OutlineCause::ConflictingComponentOffsetModes {
                    component_index,
                    flags: self.0,
                },
            ));
        }
        if component_index == 0 && !self.arguments_are_xy_values() {
            return Err(diagnostic(
                span,
                OutlineCause::FirstCompositeComponentUsesPoints,
            ));
        }
        if component_index != 0 && self.has_overlap() {
            return Err(diagnostic(
                span,
                OutlineCause::MisplacedCompositeOverlapFlag { component_index },
            ));
        }
        Ok(())
    }

    pub(super) const fn arguments_are_words(self) -> bool {
        self.contains(ARGUMENTS_ARE_WORDS)
    }

    pub(super) const fn arguments_are_xy_values(self) -> bool {
        self.contains(ARGUMENTS_ARE_XY_VALUES)
    }

    pub(super) const fn round_to_grid(self) -> bool {
        self.contains(ROUND_XY_TO_GRID)
    }

    pub(super) const fn has_scale(self) -> bool {
        self.contains(HAVE_SCALE)
    }

    pub(super) const fn has_axis_scales(self) -> bool {
        self.contains(HAVE_AXIS_SCALES)
    }

    pub(super) const fn has_matrix(self) -> bool {
        self.contains(HAVE_MATRIX)
    }

    pub(super) const fn has_more(self) -> bool {
        self.contains(MORE_COMPONENTS)
    }

    pub(super) const fn has_instructions(self) -> bool {
        self.contains(HAVE_INSTRUCTIONS)
    }

    pub(super) const fn uses_child_metrics(self) -> bool {
        self.contains(USE_MY_METRICS)
    }

    pub(super) const fn has_overlap(self) -> bool {
        self.contains(OVERLAP_COMPOUND)
    }

    pub(super) const fn uses_scaled_offset(self) -> bool {
        self.contains(SCALED_COMPONENT_OFFSET)
    }

    pub(super) const fn uses_unscaled_offset(self) -> bool {
        self.contains(UNSCALED_COMPONENT_OFFSET)
    }

    const fn contains(self, mask: u16) -> bool {
        self.0 & mask != 0
    }
}

const fn diagnostic(span: ByteSpan, cause: OutlineCause) -> Diagnostic {
    Diagnostic::at(Stage::Decode, span, Cause::Outline(cause))
}

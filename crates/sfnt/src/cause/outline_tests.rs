use super::{Cause, CoordinateAxis, OutlineCause};

#[test]
fn coordinate_axes_use_stable_outline_names() {
    assert_eq!(CoordinateAxis::Horizontal.to_string(), "x");
    assert_eq!(CoordinateAxis::Vertical.to_string(), "y");
}
use crate::LocaFormat;

#[test]
fn loca_length_failure_names_format_count_and_lengths() {
    let cause = Cause::Outline(OutlineCause::InvalidLocaLength {
        format: LocaFormat::Short,
        glyph_count: 3,
        expected: 8,
        actual: 6,
    });

    assert_eq!(
        cause.to_string(),
        "short loca length 6 does not equal 8 bytes required by 3 glyph(s)"
    );
}

#[test]
fn loca_start_failure_names_the_leading_gap() {
    let cause = Cause::Outline(OutlineCause::NonzeroLocaStart { offset: 4 });

    assert_eq!(
        cause.to_string(),
        "loca first offset 4 does not start glyf at zero"
    );
}

#[test]
fn decreasing_loca_failure_names_both_offsets() {
    let cause = Cause::Outline(OutlineCause::DecreasingLocaOffset {
        index: 2,
        offset: 4,
        previous: 8,
    });

    assert_eq!(
        cause.to_string(),
        "loca offset 2 value 4 precedes previous offset 8"
    );
}

#[test]
fn misaligned_loca_failure_names_the_entry_and_offset() {
    let cause = Cause::Outline(OutlineCause::MisalignedLocaOffset {
        index: 1,
        offset: 3,
    });

    assert_eq!(
        cause.to_string(),
        "loca offset 1 value 3 is not word aligned"
    );
}

#[test]
fn out_of_bounds_loca_failure_names_the_glyf_limit() {
    let cause = Cause::Outline(OutlineCause::LocaOffsetOutOfBounds {
        index: 1,
        offset: 12,
        glyf_length: 10,
    });

    assert_eq!(
        cause.to_string(),
        "loca offset 1 value 12 exceeds glyf length 10"
    );
}

#[test]
fn loca_end_failure_names_the_sentinel_and_glyf_length() {
    let cause = Cause::Outline(OutlineCause::LocaEndMismatch {
        index: 3,
        offset: 8,
        glyf_length: 10,
    });

    assert_eq!(
        cause.to_string(),
        "loca sentinel 3 value 8 does not equal glyf length 10"
    );
}

#[test]
fn empty_glyph_failure_distinguishes_absent_outline_data() {
    let cause = Cause::Outline(OutlineCause::EmptyGlyphDescription);

    assert_eq!(
        cause.to_string(),
        "simple glyph decoder received an empty glyph description"
    );
}

#[test]
fn composite_glyph_failure_preserves_the_signed_contour_count() {
    let cause = Cause::Outline(OutlineCause::CompositeGlyphForSimpleDecoder {
        number_of_contours: -1,
    });

    assert_eq!(
        cause.to_string(),
        "simple glyph decoder received composite contour count -1"
    );
}

#[test]
fn invalid_bounds_failure_names_both_corners() {
    let cause = Cause::Outline(OutlineCause::InvalidGlyphBounds {
        minimum_x: 9,
        minimum_y: 7,
        maximum_x: -3,
        maximum_y: -5,
    });

    assert_eq!(
        cause.to_string(),
        "glyph bounds (9, 7)..(-3, -5) are inverted"
    );
}

#[test]
fn contour_end_failure_names_both_point_indices() {
    let cause = Cause::Outline(OutlineCause::NonIncreasingContourEnd {
        contour_index: 2,
        end_point: 7,
        previous_end_point: 9,
    });

    assert_eq!(
        cause.to_string(),
        "simple contour 2 ends at point 7, not after point 9"
    );
}

#[test]
fn flag_repeat_failure_names_the_complete_expansion() {
    let cause = Cause::Outline(OutlineCause::SimpleFlagRepeatOutOfBounds {
        point_index: 4,
        additional_repetitions: 9,
        point_count: 10,
    });

    assert_eq!(
        cause.to_string(),
        "simple flag at point 4 repeats 9 additional time(s) past 10 point(s)"
    );
}

#[test]
fn reserved_flag_failure_renders_the_full_byte() {
    let cause = Cause::Outline(OutlineCause::ReservedSimpleGlyphFlag {
        point_index: 3,
        flag: 0x91,
    });

    assert_eq!(
        cause.to_string(),
        "simple flag at point 3 sets reserved bits in 0x91"
    );
}

#[test]
fn overlap_failure_names_the_late_logical_point() {
    let cause = Cause::Outline(OutlineCause::MisplacedSimpleOverlapFlag { point_index: 8 });

    assert_eq!(
        cause.to_string(),
        "simple overlap flag appears after the first physical flag at point 8"
    );
}

#[test]
fn coordinate_overflow_failure_names_axis_point_and_sum() {
    let cause = Cause::Outline(OutlineCause::SimpleCoordinateOverflow {
        axis: CoordinateAxis::Vertical,
        point_index: 12,
        previous: 32_000,
        delta: 1_000,
    });

    assert_eq!(
        cause.to_string(),
        "simple point 12 y coordinate 32000 plus delta 1000 exceeds signed design units"
    );
}

#[test]
fn glyph_bounds_mismatch_failure_names_recorded_and_calculated_rectangles() {
    let cause = Cause::Outline(OutlineCause::SimpleGlyphBoundsMismatch {
        recorded: [-4, -3, 8, 9],
        calculated: [-2, -1, 6, 7],
    });

    assert_eq!(
        cause.to_string(),
        "simple glyph bounds (-4, -3)..(8, 9) do not match points (-2, -1)..(6, 7)"
    );
}

#[test]
fn trailing_data_failure_reports_the_unconsumed_length() {
    let cause = Cause::Outline(OutlineCause::UnexpectedSimpleGlyphData { length: 3 });

    assert_eq!(
        cause.to_string(),
        "simple glyph has 3 unexpected trailing byte(s)"
    );
}

#[test]
fn padding_failure_renders_the_nonzero_byte() {
    let cause = Cause::Outline(OutlineCause::InvalidSimpleGlyphPadding { value: 0xa5 });

    assert_eq!(
        cause.to_string(),
        "simple glyph alignment byte is 0xa5, not zero"
    );
}

//! Remapping mark attachment positioning (lookup types 4, 5, and 6).
//!
//! Mark, base, ligature, and mark-2 records carry only classes and anchors, never Glyph IDs, so
//! each record array is realigned with its remapped Coverage while its contents are preserved.

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::{MarkLigaturePositioning, MarkToBasePositioning, MarkToMarkPositioning};

pub(super) fn to_base(
    input: &MarkToBasePositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> MarkToBasePositioning<NewGlyphId> {
    let (marks_coverage, marks) = super::parallel(input.mark_coverage(), input.marks(), mapping);
    let (bases_coverage, bases) = super::parallel(input.base_coverage(), input.bases(), mapping);
    MarkToBasePositioning::new(
        marks_coverage,
        bases_coverage,
        input.mark_class_count(),
        marks,
        bases,
    )
}

pub(super) fn to_ligature(
    input: &MarkLigaturePositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> MarkLigaturePositioning<NewGlyphId> {
    let (marks_coverage, marks) = super::parallel(input.mark_coverage(), input.marks(), mapping);
    let (ligatures_coverage, ligatures) =
        super::parallel(input.ligature_coverage(), input.ligatures(), mapping);
    MarkLigaturePositioning::new(
        marks_coverage,
        ligatures_coverage,
        input.mark_class_count(),
        marks,
        ligatures,
    )
}

pub(super) fn to_mark(
    input: &MarkToMarkPositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> MarkToMarkPositioning<NewGlyphId> {
    let (mark1_coverage, mark1_records) =
        super::parallel(input.mark1_coverage(), input.mark1_records(), mapping);
    let (mark2_coverage, mark2_records) =
        super::parallel(input.mark2_coverage(), input.mark2_records(), mapping);
    MarkToMarkPositioning::new(
        mark1_coverage,
        mark2_coverage,
        input.mark_class_count(),
        mark1_records,
        mark2_records,
    )
}

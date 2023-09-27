//! Conservative feasibility predicates for contextual rules.

use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::direct::{coverage_matches, coverages_match};
use crate::{ClassDefinition, ContextRule, Coverage};

pub(super) fn exact(rule: &ContextRule<GlyphId>, glyphs: &BTreeSet<GlyphId>) -> bool {
    rule.backtrack()
        .iter()
        .chain(rule.input())
        .chain(rule.lookahead())
        .all(|glyph| glyphs.contains(glyph))
}

pub(super) fn coverage(rule: &ContextRule<Coverage>, glyphs: &BTreeSet<GlyphId>) -> bool {
    coverages_match(rule.backtrack(), glyphs)
        && coverages_match(rule.input(), glyphs)
        && coverages_match(rule.lookahead(), glyphs)
}

pub(super) fn context_classes(
    rule: &ContextRule<u16>,
    first_coverage: &Coverage,
    classes: &ClassDefinition,
    glyphs: &BTreeSet<GlyphId>,
) -> bool {
    first_class_matches(first_coverage, classes, rule.input()[0], glyphs)
        && rule.input()[1..]
            .iter()
            .all(|&class| class_matches(classes, class, glyphs))
}

pub(super) fn chained_classes(
    rule: &ContextRule<u16>,
    first_coverage: &Coverage,
    backtrack: &ClassDefinition,
    input: &ClassDefinition,
    lookahead: &ClassDefinition,
    glyphs: &BTreeSet<GlyphId>,
) -> bool {
    rule.backtrack()
        .iter()
        .all(|&class| class_matches(backtrack, class, glyphs))
        && first_class_matches(first_coverage, input, rule.input()[0], glyphs)
        && rule.input()[1..]
            .iter()
            .all(|&class| class_matches(input, class, glyphs))
        && rule
            .lookahead()
            .iter()
            .all(|&class| class_matches(lookahead, class, glyphs))
}

fn first_class_matches(
    coverage: &Coverage,
    classes: &ClassDefinition,
    class: u16,
    glyphs: &BTreeSet<GlyphId>,
) -> bool {
    coverage
        .glyphs()
        .iter()
        .any(|&glyph| glyphs.contains(&glyph) && classes.class_of(glyph) == class)
}

fn class_matches(
    classes: &ClassDefinition,
    class: u16,
    glyphs: &BTreeSet<GlyphId>,
) -> bool {
    glyphs.iter().any(|&glyph| classes.class_of(glyph) == class)
}

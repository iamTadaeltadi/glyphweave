//! Remapping chained contextual positioning predicate domains (lookup type 8).

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::{ChainedContextPositioning, ContextRule, Coverage};

pub(super) fn positioning(
    input: &ChainedContextPositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> ChainedContextPositioning<NewGlyphId> {
    match input {
        ChainedContextPositioning::GlyphRules(rules) => ChainedContextPositioning::GlyphRules(
            rules
                .iter()
                .filter_map(|rule| glyph_rule(rule, mapping))
                .collect(),
        ),
        ChainedContextPositioning::ClassRules {
            coverage,
            backtrack_classes,
            input_classes,
            lookahead_classes,
            rules,
        } => ChainedContextPositioning::ClassRules {
            coverage: super::coverage(coverage, mapping),
            backtrack_classes: super::classes(backtrack_classes, mapping),
            input_classes: super::classes(input_classes, mapping),
            lookahead_classes: super::classes(lookahead_classes, mapping),
            rules: rules.clone(),
        },
        ChainedContextPositioning::CoverageRule(rule) => {
            ChainedContextPositioning::CoverageRule(coverage_rule(rule, mapping))
        }
    }
}

/// Rewrites a format 1 rule, dropping it entirely when any predicate glyph is absent.
fn glyph_rule(
    rule: &ContextRule<GlyphId>,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Option<ContextRule<NewGlyphId>> {
    let backtrack = super::glyphs(rule.backtrack(), mapping)?;
    let input = super::glyphs(rule.input(), mapping)?;
    let lookahead = super::glyphs(rule.lookahead(), mapping)?;
    let (first, remaining) = input.split_first()?;
    Some(ContextRule::chained(
        backtrack,
        *first,
        remaining.iter().copied(),
        lookahead,
        rule.records().to_vec(),
    ))
}

/// Rewrites a format 3 rule; a position whose Coverage empties simply never matches.
fn coverage_rule(
    rule: &ContextRule<Coverage<GlyphId>>,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> ContextRule<Coverage<NewGlyphId>> {
    let backtrack = coverages(rule.backtrack(), mapping);
    let input = coverages(rule.input(), mapping);
    let lookahead = coverages(rule.lookahead(), mapping);
    let (first, remaining) = input
        .split_first()
        .expect("a context rule always has a nonempty input sequence");
    ContextRule::chained(
        backtrack,
        first.clone(),
        remaining.iter().cloned(),
        lookahead,
        rule.records().to_vec(),
    )
}

fn coverages(
    input: &[Coverage],
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Vec<Coverage<NewGlyphId>> {
    input
        .iter()
        .map(|coverage| super::coverage(coverage, mapping))
        .collect()
}

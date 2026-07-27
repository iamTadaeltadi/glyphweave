//! Remapping contextual positioning predicate domains (lookup type 7).

use glyphweave_sfnt::{GlyphId, NewGlyphId};

use crate::{ContextPositioning, ContextPositioningRule, Coverage};

pub(super) fn positioning(
    input: &ContextPositioning,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> ContextPositioning<NewGlyphId> {
    match input {
        ContextPositioning::GlyphRules(rules) => ContextPositioning::GlyphRules(
            rules
                .iter()
                .filter_map(|rule| glyph_rule(rule, mapping))
                .collect(),
        ),
        ContextPositioning::ClassRules {
            coverage,
            classes,
            rules,
        } => ContextPositioning::ClassRules {
            coverage: super::coverage(coverage, mapping),
            classes: super::classes(classes, mapping),
            rules: rules.clone(),
        },
        ContextPositioning::CoverageRule(rule) => {
            ContextPositioning::CoverageRule(coverage_rule(rule, mapping))
        }
    }
}

/// Rewrites a format 1 rule, dropping it entirely when any predicate glyph is absent.
fn glyph_rule(
    rule: &ContextPositioningRule<GlyphId>,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> Option<ContextPositioningRule<NewGlyphId>> {
    let backtrack = super::glyphs(rule.backtrack(), mapping)?;
    let input = super::glyphs(rule.input(), mapping)?;
    let lookahead = super::glyphs(rule.lookahead(), mapping)?;
    let (first, remaining) = input.split_first()?;
    Some(ContextPositioningRule::chained(
        backtrack,
        *first,
        remaining.iter().copied(),
        lookahead,
        rule.records().to_vec(),
    ))
}

/// Rewrites a format 3 rule; a position whose Coverage empties simply never matches.
fn coverage_rule(
    rule: &ContextPositioningRule<Coverage<GlyphId>>,
    mapping: &mut impl FnMut(GlyphId) -> Option<NewGlyphId>,
) -> ContextPositioningRule<Coverage<NewGlyphId>> {
    let backtrack = coverages(rule.backtrack(), mapping);
    let input = coverages(rule.input(), mapping);
    let lookahead = coverages(rule.lookahead(), mapping);
    let (first, remaining) = input
        .split_first()
        .expect("a context rule always has a nonempty input sequence");
    ContextPositioningRule::chained(
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

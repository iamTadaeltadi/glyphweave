//! Retention for shared Coverage and class predicates.

use crate::{ClassDefinition, ContextRule, Coverage, GsubClosure};

pub(super) fn retain_coverage_rule(
    rule: &ContextRule<Coverage>,
    closure: &GsubClosure,
) -> Option<ContextRule<Coverage>> {
    if !super::feasible::coverage(rule, &closure.glyphs) {
        return None;
    }
    let backtrack = retain_coverages(rule.backtrack(), closure)?;
    let mut input = retain_coverages(rule.input(), closure)?.into_iter();
    let first = input.next()?;
    let lookahead = retain_coverages(rule.lookahead(), closure)?;
    Some(ContextRule::chained(
        backtrack,
        first,
        input,
        lookahead,
        rule.records().iter().copied(),
    ))
}

pub(super) fn retain_coverage(coverage: &Coverage, closure: &GsubClosure) -> Option<Coverage> {
    let retained = Coverage::from_glyphs(
        coverage
            .glyphs()
            .iter()
            .copied()
            .filter(|glyph| closure.contains(*glyph)),
    );
    (!retained.is_empty()).then_some(retained)
}

fn retain_coverages(coverages: &[Coverage], closure: &GsubClosure) -> Option<Vec<Coverage>> {
    coverages
        .iter()
        .map(|coverage| retain_coverage(coverage, closure))
        .collect()
}

pub(super) fn retain_classes(classes: &ClassDefinition, closure: &GsubClosure) -> ClassDefinition {
    ClassDefinition::from_assignments(
        classes
            .assignments()
            .filter(|(glyph, _)| closure.contains(*glyph)),
    )
}

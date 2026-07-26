//! Retention for chained contextual substitution semantics.

use crate::{ChainedContextSubstitution, GsubClosure};

pub(super) fn retain(
    value: &ChainedContextSubstitution,
    closure: &GsubClosure,
) -> Option<ChainedContextSubstitution> {
    match value {
        ChainedContextSubstitution::GlyphRules(rules) => {
            let rules = rules
                .iter()
                .filter(|rule| super::feasible::exact(rule, &closure.glyphs))
                .cloned()
                .collect::<Vec<_>>();
            (!rules.is_empty()).then_some(ChainedContextSubstitution::GlyphRules(rules))
        }
        ChainedContextSubstitution::ClassRules {
            coverage,
            backtrack_classes,
            input_classes,
            lookahead_classes,
            rules,
        } => {
            let rules = rules
                .iter()
                .filter(|rule| {
                    super::feasible::chained_classes(
                        rule,
                        coverage,
                        backtrack_classes,
                        input_classes,
                        lookahead_classes,
                        &closure.glyphs,
                    )
                })
                .cloned()
                .collect::<Vec<_>>();
            let coverage = super::predicate::retain_coverage(coverage, closure)?;
            (!rules.is_empty()).then(|| ChainedContextSubstitution::ClassRules {
                coverage,
                backtrack_classes: super::predicate::retain_classes(backtrack_classes, closure),
                input_classes: super::predicate::retain_classes(input_classes, closure),
                lookahead_classes: super::predicate::retain_classes(lookahead_classes, closure),
                rules,
            })
        }
        ChainedContextSubstitution::CoverageRule(rule) => {
            super::predicate::retain_coverage_rule(rule, closure)
                .map(ChainedContextSubstitution::CoverageRule)
        }
    }
}

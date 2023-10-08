//! Retention for non-chained contextual substitution semantics.

use crate::{ContextSubstitution, GsubClosure};

pub(super) fn retain(
    value: &ContextSubstitution,
    closure: &GsubClosure,
) -> Option<ContextSubstitution> {
    match value {
        ContextSubstitution::GlyphRules(rules) => {
            let rules = rules
                .iter()
                .filter(|rule| super::feasible::exact(rule, &closure.glyphs))
                .cloned()
                .collect::<Vec<_>>();
            (!rules.is_empty()).then_some(ContextSubstitution::GlyphRules(rules))
        }
        ContextSubstitution::ClassRules {
            coverage,
            classes,
            rules,
        } => {
            let rules = rules
                .iter()
                .filter(|rule| {
                    super::feasible::context_classes(rule, coverage, classes, &closure.glyphs)
                })
                .cloned()
                .collect::<Vec<_>>();
            let coverage = super::predicate::retain_coverage(coverage, closure)?;
            (!rules.is_empty()).then(|| ContextSubstitution::ClassRules {
                coverage,
                classes: super::predicate::retain_classes(classes, closure),
                rules,
            })
        }
        ContextSubstitution::CoverageRule(rule) => {
            super::predicate::retain_coverage_rule(rule, closure)
                .map(ContextSubstitution::CoverageRule)
        }
    }
}

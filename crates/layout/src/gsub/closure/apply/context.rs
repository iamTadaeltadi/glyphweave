//! Nested Lookup reachability from feasible contextual rules.

use std::collections::BTreeSet;

use glyphweave_sfnt::GlyphId;

use super::feasible;
use crate::{ChainedContextSubstitution, ContextSubstitution, GsubSubtable, SubstitutionRecord};

pub(super) fn apply(
    subtable: &GsubSubtable,
    glyphs: &BTreeSet<GlyphId>,
    lookups: &mut BTreeSet<u16>,
) {
    match subtable {
        GsubSubtable::Context(ContextSubstitution::GlyphRules(rules)) => {
            for rule in rules.iter().filter(|rule| feasible::exact(rule, glyphs)) {
                extend(lookups, rule.records());
            }
        }
        GsubSubtable::Context(ContextSubstitution::ClassRules {
            coverage,
            classes,
            rules,
        }) => {
            for rule in rules
                .iter()
                .filter(|rule| feasible::context_classes(rule, coverage, classes, glyphs))
            {
                extend(lookups, rule.records());
            }
        }
        GsubSubtable::Context(ContextSubstitution::CoverageRule(rule))
        | GsubSubtable::ChainedContext(ChainedContextSubstitution::CoverageRule(rule)) => {
            if feasible::coverage(rule, glyphs) {
                extend(lookups, rule.records());
            }
        }
        GsubSubtable::ChainedContext(ChainedContextSubstitution::GlyphRules(rules)) => {
            for rule in rules.iter().filter(|rule| feasible::exact(rule, glyphs)) {
                extend(lookups, rule.records());
            }
        }
        GsubSubtable::ChainedContext(ChainedContextSubstitution::ClassRules {
            coverage,
            backtrack_classes,
            input_classes,
            lookahead_classes,
            rules,
        }) => {
            for rule in rules.iter().filter(|rule| {
                feasible::chained_classes(
                    rule,
                    coverage,
                    backtrack_classes,
                    input_classes,
                    lookahead_classes,
                    glyphs,
                )
            }) {
                extend(lookups, rule.records());
            }
        }
        _ => {}
    }
}

fn extend(lookups: &mut BTreeSet<u16>, records: &[SubstitutionRecord]) {
    lookups.extend(records.iter().map(|record| record.lookup_index()));
}

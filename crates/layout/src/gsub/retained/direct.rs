//! Retention for direct substitution semantics.

use crate::{Coverage, GsubClosure, GsubSubtable};

pub(super) fn retain(subtable: &GsubSubtable, closure: &GsubClosure) -> Option<GsubSubtable> {
    match subtable {
        GsubSubtable::Single(value) => {
            let retained = value.mappings().filter(|(input, output)| {
                closure.contains(*input) && closure.contains(*output)
            });
            let retained = crate::SingleSubstitution::from_mappings(retained);
            (!retained.is_empty()).then_some(GsubSubtable::Single(retained))
        }
        GsubSubtable::Multiple(value) => {
            let retained = value.mappings().filter(|(input, outputs)| {
                closure.contains(*input) && outputs.iter().all(|glyph| closure.contains(*glyph))
            });
            let retained = crate::MultipleSubstitution::from_mappings(
                retained.map(|(input, outputs)| (input, outputs.to_vec())),
            );
            (!retained.is_empty()).then_some(GsubSubtable::Multiple(retained))
        }
        GsubSubtable::Alternate(value) => {
            let retained = value.mappings().filter_map(|(input, outputs)| {
                let outputs = outputs
                    .iter()
                    .copied()
                    .filter(|glyph| closure.contains(*glyph))
                    .collect::<Vec<_>>();
                (closure.contains(input) && !outputs.is_empty()).then_some((input, outputs))
            });
            let retained = crate::AlternateSubstitution::from_alternates(retained);
            (!retained.is_empty()).then_some(GsubSubtable::Alternate(retained))
        }
        GsubSubtable::Ligature(value) => {
            let rules = value.rules().iter().filter(|rule| {
                closure.contains(rule.replacement())
                    && rule
                        .components()
                        .iter()
                        .all(|glyph| closure.contains(*glyph))
            });
            let retained = crate::LigatureSubstitution::new(rules.cloned());
            (!retained.is_empty()).then_some(GsubSubtable::Ligature(retained))
        }
        GsubSubtable::ReverseChainSingle(value) => {
            let backtrack = retain_coverages(value.backtrack(), closure)?;
            let lookahead = retain_coverages(value.lookahead(), closure)?;
            let mappings = value.substitutions().mappings().filter(|(input, output)| {
                closure.contains(*input) && closure.contains(*output)
            });
            let substitutions = crate::SingleSubstitution::from_mappings(mappings);
            (!substitutions.is_empty())
                .then(|| crate::ReverseChainSingleSubstitution::new(backtrack, substitutions, lookahead))
                .map(GsubSubtable::ReverseChainSingle)
        }
        _ => None,
    }
}

fn retain_coverages(coverages: &[Coverage], closure: &GsubClosure) -> Option<Vec<Coverage>> {
    coverages
        .iter()
        .map(|coverage| {
            let retained = Coverage::from_glyphs(
                coverage.glyphs().iter().copied().filter(|glyph| closure.contains(*glyph)),
            );
            (!retained.is_empty()).then_some(retained)
        })
        .collect()
}

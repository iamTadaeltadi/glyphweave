//! Canonical Glyph Substitution rules, closure, retention, and remapping.

mod alternate;
mod chained_context_substitution;
mod closure;
mod context_rule;
mod context_substitution;
mod decode;
mod ligature_rule;
mod ligature_substitution;
mod lookup;
mod lookup_list;
mod multiple;
mod retained;
mod reverse_chain;
mod sequence_map;
mod single;
mod substitution_record;
mod subtable;

pub use alternate::AlternateSubstitution;
pub use chained_context_substitution::ChainedContextSubstitution;
pub use closure::GsubClosure;
pub use context_rule::ContextRule;
pub use context_substitution::ContextSubstitution;
pub use ligature_rule::LigatureRule;
pub use ligature_substitution::LigatureSubstitution;
pub use lookup::GsubLookup;
pub use lookup_list::GsubLookupList;
pub use multiple::MultipleSubstitution;
pub use retained::RetainedGsub;
pub use reverse_chain::ReverseChainSingleSubstitution;
pub use single::SingleSubstitution;
pub use substitution_record::SubstitutionRecord;
pub use subtable::GsubSubtable;

//! Canonical Glyph Substitution rules, closure, retention, and remapping.

mod alternate;
mod decode;
mod ligature_rule;
mod ligature_substitution;
mod multiple;
mod reverse_chain;
mod sequence_map;
mod single;
mod substitution_record;

pub use alternate::AlternateSubstitution;
pub use ligature_rule::LigatureRule;
pub use ligature_substitution::LigatureSubstitution;
pub use multiple::MultipleSubstitution;
pub use reverse_chain::ReverseChainSingleSubstitution;
pub use single::SingleSubstitution;
pub use substitution_record::SubstitutionRecord;

//! Canonical Glyph Substitution rules, closure, retention, and remapping.

mod multiple;
mod sequence_map;
mod single;
mod substitution_record;

pub use multiple::MultipleSubstitution;
pub use single::SingleSubstitution;
pub use substitution_record::SubstitutionRecord;

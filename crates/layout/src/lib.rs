//! OpenType script, feature, lookup, coverage, and class-definition transformations.
//!
//! Layout retention remains independent from outline decoding and communicates through SFNT
//! primitives and explicit glyph selections.

#![forbid(unsafe_code)]

mod class_definition;
mod coverage;

pub use class_definition::ClassDefinition;
pub use coverage::Coverage;

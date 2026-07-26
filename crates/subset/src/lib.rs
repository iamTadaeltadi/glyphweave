//! Deterministic subset planning and orchestration of pure font-table passes.
//!
//! This crate computes selections, closure, retention, and remapping without performing I/O or
//! encoding final SFNT bytes.

#![forbid(unsafe_code)]

mod codepoint_set;
mod retention;

pub use codepoint_set::CodepointSet;
pub use retention::Retention;

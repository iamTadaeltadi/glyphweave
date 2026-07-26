//! TrueType outline decoding, composite glyph closure, remapping, and rewriting.
//!
//! Outline transformations are pure and refer to source locations through checked SFNT byte
//! spans.

#![forbid(unsafe_code)]

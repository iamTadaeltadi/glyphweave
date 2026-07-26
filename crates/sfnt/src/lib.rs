//! Bounded decoding primitives for SFNT containers, faces, table directories, and scalar tables.
//!
//! This crate is the dependency root of Glyphweave's computational pipeline. It performs no I/O
//! and exposes no unchecked view into input bytes.

#![forbid(unsafe_code)]

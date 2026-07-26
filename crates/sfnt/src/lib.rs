//! Bounded decoding primitives for SFNT containers, faces, table directories, and scalar tables.
//!
//! This crate is the dependency root of Glyphweave's computational pipeline. It performs no I/O
//! and exposes no unchecked view into input bytes. Every value decoded here is produced by the
//! bounded [`Reader`], located by a [`ByteSpan`], and reported through a span-aware
//! [`Diagnostic`].

#![forbid(unsafe_code)]

mod diagnostic;
mod reader;
mod span;
mod tag;

pub use diagnostic::{Cause, Decoded, Diagnostic, Severity, Stage};
pub use reader::Reader;
pub use span::ByteSpan;
pub use tag::Tag;

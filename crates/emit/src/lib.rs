//! Canonical table encoding, directory assembly, alignment, and checksum calculation.
//!
//! Emission consumes validated subset artifacts and returns immutable font bytes; it does not
//! write files.

#![forbid(unsafe_code)]

# ADR 0001: Use a pass-based font-subsetting pipeline

- Status: Accepted
- Date: 2026-07-26

## Context

Glyphweave transforms an immutable font plus a selection request into a new font. Correctness depends on ordered decoding, closure, remapping, rewriting, emission, and independent verification. The computational work does not require I/O, clocks, randomness, or asynchronous execution.

## Decision

Use a pass-based architecture with seven crates and the dependency graph in `ARCHITECTURE.md`. Pipeline stages accept immutable values and explicit options, and return a new value or span-aware Diagnostics. Only `glyphweave-cli` crosses filesystem, process, environment, or reference-tool boundaries.

The first release targets TrueType-flavoured OpenType fonts. FontTools and HarfBuzz provide differential semantic references, while Glyphweave's verifier is implemented independently from its emitter.

## Rejected options

- **Hexagonal domain/application/adapters:** useful for request/response services, but it hides the real ordering and artifacts of a compiler pipeline.
- **One monolithic crate:** initially convenient, but it would make independent verification and dependency enforcement difficult.
- **Streaming rewrite:** many tables require the final Glyph Closure and Remap before offsets or records can be emitted.
- **CFF and variable fonts in 1.0:** these add different outline and delta models before the TrueType pipeline is proved.

## Consequences

- Every transformation boundary is directly testable and reproducible.
- Memory use includes immutable input plus intermediate artifacts; later optimization must preserve the pure-stage contract.
- Layout and outline crates can evolve independently but share only SFNT primitives.
- Verification can catch defects in emission because it cannot depend on emitter or subset internals.


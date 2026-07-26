# ADR 0005: Make glyph remapping total, dense, and type-distinct

- Status: Accepted
- Date: 2026-07-26

## Context

Subsetting retains a sparse set of input glyph indices and emits those glyphs into a dense output
index space. Treating both spaces as raw integers makes it easy to write an input Glyph ID into an
emitted table after remapping. Later outline, character-map, and layout passes also need one shared
definition of the retained set and a deterministic way to enumerate it.

OpenType Glyph IDs are unsigned 16-bit values. Glyph zero is `.notdef`; it must always be retained
and must remain output glyph zero. A useful Remap must be defined for every retained Glyph ID,
undefined for every discarded Glyph ID, injective, and dense from zero.

## Decision

Define `GlyphId` and `NewGlyphId` as distinct `u16` newtypes in `glyphweave-sfnt`. Neither type
implicitly converts to the other. Both expose explicit constructors and raw values, and each names
zero as `.notdef` in its own index space.

Define `GlyphSet` and `Remap` in `glyphweave-outline`. A Glyph Set stores unique Glyph IDs in
ascending order and includes `.notdef` from construction onward. Public mutation cannot remove
`.notdef`. Iteration is deterministic and set operations preserve the invariant.

Construct a Remap only from a Glyph Set. Ascending retained Glyph IDs receive consecutive New
Glyph IDs, so `.notdef` maps to zero and identical sets always produce identical mappings. Store
both forward and reverse relations so table rewriting and independent invariant checks do not need
to reconstruct either direction. Construction is infallible because the complete `u16` Glyph ID
universe contains exactly as many values as the output `u16` space.

## Consequences

- Rust's type checker rejects accidental use of an input Glyph ID where an emitted ID is required.
- Every retained reference has a mapping, while discarded references return `None` explicitly.
- Remap construction is canonical and independent of insertion order.
- A full 65,536-glyph font remains representable, with the last New Glyph ID equal to 65,535.
- Memory for forward lookup scales with the greatest retained input ID; reverse lookup scales with
  the retained Glyph Set.

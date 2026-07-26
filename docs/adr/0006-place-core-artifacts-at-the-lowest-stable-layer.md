# ADR 0006: Place core artifacts at the lowest stable layer

- Status: Accepted
- Date: 2026-07-26

## Context

Milestone 1.5 introduces five domain models used across later decoding, planning, emission, and
verification stages. Placing them in a convenience crate would either invert the frozen dependency
graph or make independent verification depend on the implementation it is meant to check.

A resolved Character Map must have deterministic lookup semantics. OpenType defines glyph zero as
the missing-character result for codes without a supported glyph. A Subset Plan must also prevent
its retained Glyph Set and dense Remap from disagreeing. A Font Artifact must be shareable across
pipeline boundaries without exposing mutable bytes.

## Decision

Define `CharacterMap` and `FontArtifact` in `glyphweave-sfnt`, the common dependency root used by
decoders, emitters, and independent verifiers. Define `CodepointSet`, `Retention`, and `SubsetPlan`
in `glyphweave-subset`, the orchestration layer.

Represent Unicode scalar values with Rust `char`, which excludes surrogate code points by
construction. Codepoint Sets and Character Maps use ordered standard-library collections so their
iteration is canonical and independent of insertion order.

A Character Map stores only mappings to nonzero Glyph IDs. Lookup is total: an absent scalar maps
to `GlyphId::NOTDEF`. Inserting glyph zero canonicalizes to removing the explicit entry. Unicode
variation-sequence state will extend this model when format 14 is implemented; the base mapping
remains unchanged.

`Retention` is a closed three-way decision: drop bytes, keep bytes unchanged, or rewrite a decoded
structure. An omitted table decision defaults to drop. A Subset Plan owns its requested Codepoint
Set, final retained Glyph Set, canonical per-Tag Retention decisions, and a Remap constructed
internally from that Glyph Set. Callers cannot inject a mismatched Remap.

A Font Artifact owns an immutable reference-counted byte slice. Cloning an artifact shares bytes,
and the public API exposes only immutable byte access. Its debug representation reports size rather
than dumping potentially unbounded font data.

## Consequences

- Unicode surrogate values cannot enter a Codepoint Set or Character Map through safe APIs.
- Missing character mappings have one canonical representation and always resolve to `.notdef`.
- Identical logical requests and table decisions iterate in identical order.
- Every Subset Plan carries a total dense Remap for exactly its retained Glyph Set.
- Emit and verify can exchange Font Artifacts without depending on one another or copying bytes.
- Later format-14 and Table Pass work can add behavior without moving these foundational types.

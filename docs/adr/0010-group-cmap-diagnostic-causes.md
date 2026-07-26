# ADR 0010: Group cmap diagnostic causes

- Status: Accepted
- Date: 2026-07-26

## Context

`Cause` began as the complete SFNT diagnostic taxonomy. Collection, directory, bounded-read, and
character-map failures therefore shared one flat enum. Formats 4 and 12 added enough structured
character-map failures that `cause.rs` reached 382 lines before format 14 was implemented. Adding
the format-14 header, offset, selector, range, mapping, and overlap failures to that enum would
cross the repository's 400-line cohesion-review threshold and leave unrelated domains interleaved.

The public `Diagnostic` type still needs one stable cause value. Diagnostics must also remain easy
to compare in tests and exhaustive within each implemented domain, without macros or generated
source hiding the taxonomy.

## Decision

Define a public, non-exhaustive `CmapCause` enum in `cause/cmap.rs`. Move every format-specific
character-map failure into it. Replace the flat cmap variants in `Cause` with one
`Cause::Cmap(CmapCause)` variant, and export both types from `glyphweave-sfnt`.

Keep `Cause` as the value stored by `Diagnostic`; stages, severity, and Byte Spans do not change.
Delegate human-readable formatting to the nested cause. Decoder call sites construct the category
explicitly so that a diagnostic's subsystem is visible at the point of failure.

Apply this extraction only to cmap now. Other cause families remain flat until their own size or
cohesion pressure justifies a separately recorded extraction. Do not use `include!`, variant-list
macros, erased strings, or numeric error codes to split the source mechanically.

## Consequences

- Character-map diagnostics gain a cohesive namespace that can grow through formats 14 and later
  outer-table precedence work without inflating the top-level taxonomy.
- Existing pre-1.0 callers that construct or pattern-match a cmap `Cause` must add the explicit
  `CmapCause` layer; rendered messages and located Diagnostics remain stable.
- Every format-specific variant remains typed, documented, directly testable, and visible to Rust
  tooling.
- Collection and directory diagnostics are not refactored speculatively.

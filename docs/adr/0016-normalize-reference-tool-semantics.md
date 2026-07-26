# ADR 0016: Normalize reference-tool semantics at the CLI test boundary

- Status: Accepted
- Date: 2026-07-26

## Context

M2.8 must compare character maps and TrueType outlines with FontTools and HarfBuzz. Depending on
fonts installed by an operating system would make the suite non-reproducible, while comparing
serialized font bytes would reject valid differences in table ordering, packing, and checksums.
The architecture also permits process execution only at the CLI boundary.

Reference tools are intentionally optional developer dependencies. Requiring them for every
workspace build would couple the memory-safe computational crates to Python or native binaries.

## Decision

Check in a small deterministic TrueType fixture generated from a reviewed FontTools script. The
fixture covers BMP and supplementary character maps, variation sequences, empty and simple
outlines, direct composites, nested composites, component transforms, and closure sharing. Keep a
normalized semantic manifest beside it so ordinary Rust tests remain deterministic without an
external program.

Place live differential tests in `glyphweave-cli` integration tests. When present, `ttx`,
`pyftsubset`, and `hb-subset` are invoked as reference processes and their output is normalized to
Glyph IDs, Unicode mappings, outline classifications, component edges, and retained closure. A
missing executable causes that reference case to report an explicit skip rather than fail the
workspace gate. A present tool that exits unsuccessfully or disagrees semantically fails the test.

Compare semantic models, never complete output bytes. Use only temporary paths derived for one
test process and remove each produced artifact after inspection. The checked-in generator remains
the source for fixture evolution; binary fixture changes and normalized manifest changes are
reviewed together.

## Consequences

- Local and CI tests exercise the same immutable font even when reference tools are unavailable.
- Developers with FontTools or HarfBuzz installed receive live differential validation.
- Reference-process I/O does not leak into `glyphweave-sfnt` or `glyphweave-outline`.
- Equivalent reference encodings do not create false failures.
- Adding a new fixture shape requires extending both the generator and semantic manifest.

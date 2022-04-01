# ARCHITECTURE

Glyphweave is a transformation pipeline. Architecture changes require an ADR; crate dependencies must remain acyclic and follow the graph below.

```text
                    +--------------------+
                    |   glyphweave-cli   |  filesystem and process boundary
                    +----------+---------+
                               |
          +--------------------+--------------------+
          |                    |                    |
          v                    v                    v
 +------------------+ +----------------+ +------------------+
 | glyphweave-subset| |glyphweave-emit | |glyphweave-verify |
 +---------+--------+ +--------+-------+ +---------+--------+
           |                   |                   |
           +----------+--------+-------------------+
                      |
             +--------+--------+
             |                 |
             v                 v
 +------------------+ +------------------+
 |glyphweave-outline| | glyphweave-layout|
 +---------+--------+ +---------+--------+
           |                    |
           +----------+---------+
                      v
             +----------------+
             |glyphweave-sfnt |
             +----------------+
```

## Pipeline

The CLI reads bytes and constructs an immutable Font Artifact. Pure stages then perform:

1. SFNT and Face decoding.
2. Codepoint Set resolution into an initial Glyph Set.
3. Outline and layout Glyph Closure.
4. Dense Glyph ID Remap construction.
5. Independent Table Pass transformations.
6. Canonical SFNT emission.
7. Independent Verification and optional Shape Probe comparison.

A stage receives immutable input plus explicit options and returns a new artifact or typed Diagnostics. Stages do not read files, inspect environment variables, use a clock, generate randomness, spawn tasks, or write output. The CLI owns all I/O and converts external failures to exit status and user-facing reports.

## Crate responsibilities

### `glyphweave-sfnt`

Defines Tag, Byte Span, Glyph ID, Table Directory, Table Record, Face, Font Collection, Character Map, and bounded big-endian decoding. It depends only on the standard library.

### `glyphweave-outline`

Defines Outline, Simple Glyph, Composite Glyph, Component, Glyph Set, Glyph Closure, Loca, Glyf, and outline-specific remapping. It may depend only on `glyphweave-sfnt`.

### `glyphweave-layout`

Defines Layout Table, Script List, Feature List, Lookup List, Coverage, and Class Definition parsing and transformations. It may depend only on `glyphweave-sfnt`.

### `glyphweave-subset`

Defines Codepoint Set, Retention, Remap, Subset Plan, Table Pass scheduling, and pipeline Diagnostics. It coordinates `sfnt`, `outline`, and `layout` without I/O.

### `glyphweave-emit`

Encodes transformed tables, applies Canonical Order, lays out the Table Directory, calculates Checksum values, and returns a Font Artifact. It may depend on `sfnt` and `subset`.

### `glyphweave-verify`

Re-decodes a Font Artifact and performs structural, referential, and semantic Verification without reusing emitter internals. It may depend on `sfnt`, `outline`, and `layout`, but never `emit` or `subset`.

### `glyphweave-cli`

Owns file reads, atomic writes, argument parsing, selection syntax, formatted Diagnostics, optional reference-process execution, and exit codes. It is the only crate allowed to perform I/O or invoke external programs.

## Dependency and code rules

- No dependency may point upward in the graph and no cyclic feature relationship is allowed.
- The six library crates forbid unsafe code, async runtimes, filesystem access, clocks, environment access, and randomness.
- Input bytes remain immutable. A Byte Span, not a borrowed subslice hidden in a domain type, identifies source locations.
- Diagnostics carry the pipeline stage, domain-specific cause, and Byte Span when the failure originates in input.
- Public APIs use domain vocabulary from `PROJECT.md`; banned generic names from `BLUEPRINT.md` fail review.
- A source file must stay below 500 lines. Crossing 400 lines triggers a cohesion review before more code is added.
- Parsing and encoding implementations remain independent enough for Verification to catch emitter defects.

## Test topology

- Unit tests cover typed invariants and every Diagnostic variant.
- Property tests generate tables, glyph graphs, mappings, and pass sequences.
- Golden fixtures cover published and minimized real-world table shapes.
- Fuzz targets attack SFNT, `cmap`, `glyf`, GSUB, and GPOS decoding.
- Differential tests compare semantic output with FontTools and HarfBuzz when those tools are present.
- End-to-end tests execute the built CLI against fixture fonts and verify the emitted artifact independently.

Coverage floors are line ≥ 80%, branch ≥ 70%, and computational core ≥ 90%. Mutation-test survivors must be killed or justified in writing.


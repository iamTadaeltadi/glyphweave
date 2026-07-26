# PROGRESS

Exactly one checklist item carries the next-work marker. Completed items are never reordered; actual line, test, and coverage figures are appended when known.

## M0 — Repository foundation (budget: ~300 source lines)

- [x] 0.1 Freeze the Domain Card and 40-term glossary — documentation only
- [x] 0.2 Freeze the source/test budget and pass-based architecture — documentation only
- [x] 0.3 Scaffold seven crates, pinned MSRV, workspace lints, quality command, and three-OS CI — 42 source lines
- [x] 0.4 Record ADR-0001 and install the deterministic session protocol
- [x] 0.5 Pass the local MSRV quality gate — 0 behavior tests, coverage N/A for scaffold-only crates
- [ ] 0.6 Connect a Git remote, push, confirm three-OS CI, and tag `v0.0.1` <-- NEXT

## M1 — Core data model and typed invariants (budget: 3,700 lines)

- [ ] 1.1 Add Tag, Byte Span, bounded big-endian reads, and span-aware Diagnostics
- [ ] 1.2 Decode and validate Table Directory records
- [ ] 1.3 Decode Font Collections and select a Face by Face Index
- [ ] 1.4 Introduce Glyph ID, New Glyph ID, Glyph Set, and the total dense Remap
- [ ] 1.5 Model Codepoint Set, Character Map, Retention, Subset Plan, and Font Artifact
- [ ] 1.6 Establish property generators and prove M1 invariants

## M2 — Character maps and TrueType outlines (budget: 4,800 lines)

- [ ] 2.1 Decode and resolve `cmap` format 4
- [ ] 2.2 Decode and resolve `cmap` format 12
- [ ] 2.3 Decode Variation Selector mappings in `cmap` format 14
- [ ] 2.4 Decode Loca offsets in short and long forms
- [ ] 2.5 Decode Simple Glyph contours, flags, and coordinate deltas
- [ ] 2.6 Decode Composite Glyph components and transforms
- [ ] 2.7 Compute cycle-safe composite Glyph Closure
- [ ] 2.8 Differentially test character maps and outlines against reference tools

## M3 — OpenType layout (budget: 4,300 lines)

- [ ] 3.1 Decode and encode Coverage and Class Definition structures
- [ ] 3.2 Decode Script List, language systems, and Feature List
- [ ] 3.3 Decode Lookup List flags, extensions, and mark filtering
- [ ] 3.4 Implement supported GSUB retention, closure, and remapping
- [ ] 3.5 Implement supported GPOS retention and remapping
- [ ] 3.6 Prune empty layout paths and repair all indices
- [ ] 3.7 Differentially probe layout behavior with HarfBuzz

## M4 — Subset planning and table rewriting (budget: 4,100 lines)

- [ ] 4.1 Validate requests and produce the initial Glyph Set
- [ ] 4.2 Compute cross-table Glyph Closure to a fixed point
- [ ] 4.3 Construct the dense deterministic Remap
- [ ] 4.4 Rewrite Glyf, Loca, and Horizontal Metric records
- [ ] 4.5 Rewrite Character Maps and scalar profile tables
- [ ] 4.6 Rewrite names, Unicode ranges, and post metadata
- [ ] 4.7 Enforce table Retention and opaque-table declarations

## M5 — Canonical emission and CLI (budget: 2,900 lines)

- [ ] 5.1 Encode transformed tables in Canonical Order
- [ ] 5.2 Assemble aligned Table Directory records and Checksum values
- [ ] 5.3 Apply whole-font checksum adjustment and emit a Font Artifact
- [ ] 5.4 Implement CLI selection syntax and deterministic reporting
- [ ] 5.5 Add bounded reads, atomic writes, and defined exit behavior
- [ ] 5.6 Add built-binary end-to-end fixtures

## M6 — Independent verification and observability (budget: 1,500 lines)

- [ ] 6.1 Verify directories, spans, alignment, overlap, and checksums independently
- [ ] 6.2 Verify outline, character-map, and layout references independently
- [ ] 6.3 Add structured pipeline timing and size-delta reports at the CLI boundary
- [ ] 6.4 Add inspect, verify, and differential Shape Probe commands

## M7 — Hardening, documentation, and release (budget: remainder to 21,300)

- [ ] 7.1 Add parser fuzz targets and minimize the seed corpus
- [ ] 7.2 Reach repository and computational-core coverage floors
- [ ] 7.3 Run mutation testing and document every justified survivor
- [ ] 7.4 Benchmark representative Latin, Arabic, CJK, and collection fonts
- [ ] 7.5 Complete public API docs, format notes, and operational guides
- [ ] 7.6 Pass full self-critique, publish 1.0 artifacts, and tag `v1.0.0`

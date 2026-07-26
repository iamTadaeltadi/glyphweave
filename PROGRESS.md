# PROGRESS

Exactly one checklist item carries the next-work marker. Completed items are never reordered; actual line, test, and coverage figures are appended when known.

## M0 — Repository foundation (budget: ~300 source lines)

- [x] 0.1 Freeze the Domain Card and 40-term glossary — documentation only
- [x] 0.2 Freeze the source/test budget and pass-based architecture — documentation only
- [x] 0.3 Scaffold seven crates, pinned MSRV, workspace lints, quality command, and three-OS CI — 42 source lines
- [x] 0.4 Record ADR-0001 and install the deterministic session protocol
- [x] 0.5 Pass the local MSRV quality gate — 0 behavior tests, coverage N/A for scaffold-only crates
- [x] 0.6 Connect a Git remote, push, confirm three-OS CI, and tag `v0.0.1` — pushed to private `iamTadaeltadi/glyphweave`; remote Actions billing-blocked, local `just check` gate green (ADR-0002); tagged `v0.0.1`

## M1 — Core data model and typed invariants (budget: 3,700 lines)

- [x] 1.1 Add Tag, Byte Span, bounded big-endian reads, and span-aware Diagnostics — 614 source / 302 test lines, 28 unit + 4 doc tests, 99% line / 98% region cov (budget 520; +94 from mandatory doc lines)
- [x] 1.2 Decode and validate Table Directory records — 638 source / 649 test lines, 24 unit + 21 integration + 4 doc tests, 99% line / 94% region cov (budget 620; +18 for checked span and diagnostic edge handling)
- [x] 1.3 Decode Font Collections and select a Face by Face Index — 959 source / 983 test lines, 24 unit + 27 integration + 3 doc tests, 97% line / 94% region cov (budget 420; +539 for eager collection-wide directory, shared-table, and DSIG span validation with typed diagnostics)
- [x] 1.4 Introduce Glyph ID, New Glyph ID, Glyph Set, and the total dense Remap — 311 source / 214 test lines, 16 unit + 1 integration + 4 doc tests, 100% line / 100% region cov (budget 360; -49 through shared compact scalar types)
- [x] 1.5 Model Codepoint Set, Character Map, Retention, Subset Plan, and Font Artifact — 437 source / 296 test lines, 21 unit + 1 integration + 5 doc tests, 100% line / 100% region cov for new core (M1 cumulative 2,959 / 3,700 source lines)
- [x] 1.6 Establish property generators and prove M1 invariants — 0 source / 1,051 test lines, 28 fixed-seed property tests, 97.75% line / 94.79% region cov across M1 core (M1 total 2,959 source / 3,495 test lines)

## M2 — Character maps and TrueType outlines (budget: 4,800 lines)

- [x] 2.1 Decode and resolve `cmap` format 4 — 530 source / 702 test lines, 35 unit + integration + 3 fixed-seed property + 1 doc tests, 98.46% line / 88.75% region cov in decoder (SFNT crate 97.71% line / 93.64% region cov)
- [x] 2.2 Decode and resolve `cmap` format 12 — 362 source / 492 test lines, 26 unit + integration + 2 fixed-seed property + 1 doc tests, 98.58% line / 92.39% region cov in decoder (SFNT crate 97.84% line / 93.69% region cov)
- [x] 2.3 Decode Variation Selector mappings in `cmap` format 14 — 1,270 source / 1,591 test lines, 64 unit + integration + 2 fixed-seed property + 4 doc tests, 98.96% line / 92.21% region cov in decoder (SFNT crate 97.89% line / 93.53% region cov)
- [x] 2.4 Decode Loca offsets in short and long forms — 386 source / 658 test lines, 32 unit + integration + 2 fixed-seed property + 2 doc tests, 99.07% line / 92.16% region cov in decoder (outline crate 99.56% line / 90.65% region cov)
- [x] 2.5 Decode Simple Glyph contours, flags, and coordinate deltas — 902 source / 1,475 test lines, 70 unit + integration + 4 fixed-seed property + 2 doc tests, 98.87% line / 96.41% region cov across the decoder (outline crate 99.37% line / 94.33% region cov)
- [x] 2.6 Decode Composite Glyph components and transforms — 765 source / 1,310 test lines, 59 unit + integration + 3 fixed-seed property + 5 doc tests, 99.13% line / 90.30% region cov across the decoder (outline crate 99.36% line / 93.33% region cov)
- [x] 2.7 Compute cycle-safe composite Glyph Closure — 470 source / 922 test lines, 32 unit + integration + 3 fixed-seed property + 2 doc tests, 100% line / 100% region cov across the eager `glyf` decoder and closure engine
- [x] 2.8 Differentially test character maps and outlines against reference tools — 0 source / 816 test lines, 8 deterministic + 14 live differential tests against FontTools 4.55.3 and HarfBuzz 10.4.0, with explicit unavailable-tool skips

## M3 — OpenType layout (budget: 4,300 lines)

- [x] 3.1 Decode and encode Coverage and Class Definition structures — 654 source / 962 test lines, 63 unit + integration + 7 fixed-seed property + 2 doc tests, 100% line / 100% region cov across the canonical decoders and shortest-format encoders
- [ ] 3.2 Decode Script List, language systems, and Feature List <-- NEXT
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

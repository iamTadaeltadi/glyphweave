# PROJECT

Name:          Glyphweave
One sentence:  A deterministic Rust compiler that turns TrueType-flavoured OpenType fonts into smaller, structurally valid fonts containing only the glyphs and layout behavior a caller requests.
Domain shape:  Transformation pipeline
Architecture:  Pass-based `Stage<In, Out>` chain with immutable inputs, explicit artifacts, and span-carrying diagnostics
Source target: 21,300 lines
Test target:   20,300 lines (0.95 × source)
Crates:
- `glyphweave-sfnt` — bounded decoding of collections, faces, directories, character maps, and scalar tables.
- `glyphweave-outline` — TrueType outline decoding, composite closure, glyph remapping, and outline rewriting.
- `glyphweave-layout` — OpenType script, feature, lookup, coverage, and class-definition retention.
- `glyphweave-subset` — pure orchestration that derives and executes a deterministic subset plan.
- `glyphweave-emit` — canonical binary encoding, table assembly, offsets, padding, and checksums.
- `glyphweave-verify` — independent structural and referential verification of emitted font artifacts.
- `glyphweave-cli` — filesystem I/O, command-line selection syntax, reporting, and process exit behavior.
MSRV:          1.85.0

## Glossary

- **SFNT** — the table-directory container used by TrueType and OpenType fonts.
- **Font Collection** — one container holding multiple faces, with tables that may be shared.
- **Face** — one font selected from an SFNT file or font collection.
- **Face Index** — the zero-based selector for a face in a font collection.
- **Table Directory** — the ordered set of records locating a face's tables.
- **Table Record** — a tag, checksum, byte offset, and byte length for one table.
- **Tag** — a four-byte identifier such as `cmap`, `glyf`, or `GSUB`.
- **Byte Span** — a checked half-open range into an immutable input or output artifact.
- **Checksum** — the OpenType unsigned-word sum used for tables and whole-font adjustment.
- **Glyph ID** — an input font's unsigned glyph index.
- **New Glyph ID** — a glyph index in the emitted font, distinct from an input Glyph ID.
- **.notdef** — glyph zero, which is always retained and remains glyph zero.
- **Codepoint Set** — the requested Unicode scalar values.
- **Glyph Set** — the retained Glyph IDs before remapping.
- **Character Map** — the resolved relation from Unicode scalar values to Glyph IDs.
- **Cmap Subtable** — one encoded mapping within the `cmap` table.
- **Outline** — the contours or components that define a glyph's shape.
- **Simple Glyph** — a TrueType outline encoded directly as points and contours.
- **Composite Glyph** — a TrueType outline assembled from transformed component glyphs.
- **Component** — one referenced glyph and transform inside a Composite Glyph.
- **Glyph Closure** — the smallest Glyph Set containing every transitively required glyph.
- **Retention** — the decision to keep, rewrite, or drop a table or record.
- **Remap** — the total mapping from retained Glyph IDs to dense New Glyph IDs.
- **Loca** — the `loca` table's offsets into `glyf` data.
- **Glyf** — the `glyf` table containing TrueType outlines.
- **Horizontal Metric** — an advance width and left side bearing from `hmtx`.
- **Layout Table** — `GSUB` or `GPOS`, containing scripts, features, and lookups.
- **Script List** — the layout records selecting language systems for a writing script.
- **Feature List** — the layout records grouping lookup behavior by feature tag.
- **Lookup List** — the ordered transformations referenced by layout features.
- **Coverage** — a sorted set or range encoding of glyphs affected by a lookup.
- **Class Definition** — a compact mapping from glyphs to layout class numbers.
- **Variation Selector** — a Unicode scalar that chooses a variant glyph through `cmap` format 14.
- **Subset Plan** — the immutable, validated description of selections and table passes.
- **Table Pass** — one pure transformation over a named table or related table family.
- **Font Artifact** — an immutable byte sequence produced at a pipeline boundary.
- **Diagnostic** — a typed failure or warning tied to a Byte Span and pipeline stage.
- **Verification** — independent proof checks over an emitted Font Artifact.
- **Shape Probe** — a small text, script, language, and feature tuple used for semantic comparison.
- **Canonical Order** — deterministic ordering used where the font format permits equivalent encodings.

## Irreducible capabilities

1. Decode standalone SFNT faces and indexed faces in TrueType Collections without unchecked reads.
2. Resolve Unicode requests through `cmap` formats 4, 12, and 14 with explicit precedence.
3. Decode simple and composite TrueType outlines, rejecting malformed offsets and component cycles.
4. Compute transitive Glyph Closure across Composite Glyphs and layout substitutions.
5. Produce a dense, deterministic Remap while pinning .notdef to New Glyph ID zero.
6. Retain and rewrite horizontal metrics, headers, locations, profiles, names, and Unicode ranges.
7. Prune and remap OpenType script, language, feature, lookup, Coverage, and Class Definition records.
8. Rebuild Unicode character maps using the smallest supported encoding that preserves semantics.
9. Assemble aligned SFNT tables with correct per-table checksums and whole-font checksum adjustment.
10. Preserve opted-in unknown tables byte-for-byte when their glyph independence is declared.
11. Verify all offsets, lengths, references, glyph bounds, checksums, and required tables independently of emission.
12. Expose deterministic CLI selection, inspection, verification, and differential-probe workflows.

## Invariants

1. Every Byte Span is in bounds before bytes can be observed through it.
2. .notdef is always retained and `GlyphId(0)` always remaps to `NewGlyphId(0)`.
3. A Remap is total over the Glyph Set, injective, dense from zero, and undefined outside that set.
4. Glyph Closure contains every Composite Glyph component and every retained substitution output.
5. No emitted Coverage, Class Definition, Component, character mapping, or layout record references a missing New Glyph ID.
6. Table records are four-byte aligned, non-overlapping, in bounds, and match their declared checksums.
7. Canonical Order makes identical input bytes plus an identical Subset Plan produce identical output bytes.
8. Verification never trusts structures or decoded values created by the emission path.

## Reference implementations

- The OpenType specification is the normative source for table structure, validation, and checksum behavior.
- FontTools `pyftsubset` is the primary differential reference for retained Unicode mappings, glyph closure, and table presence.
- HarfBuzz `hb-subset` and shaping output are the secondary reference for layout closure and Shape Probe equivalence.
- Glyphweave differs by offering a memory-safe, sans-I/O Rust core; stable stage diagnostics; an independently implemented verifier; and deterministic artifacts as a contractual property. Byte-for-byte parity with either reference is not required when the result is semantically equivalent.

## Non-goals

- CFF, CFF2, Type 1, bitmap-only, SVG, or AAT outline and layout formats in 1.0.
- Rewriting variable-font delta stores or producing static instances.
- Interpreting, optimizing, or synthesizing TrueType hinting bytecode.
- WOFF or WOFF2 container decoding and encoding.
- Font design, contour editing, autohinting, rasterization, or text shaping.
- Network APIs, background daemons, registries, plugin systems, or a graphical interface.
- Repairing malformed fonts by guessing author intent; failures remain explicit Diagnostics.


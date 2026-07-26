# BUDGET

This budget is frozen with M0. Actual line counts are recorded in `PROGRESS.md`; deviations are explained there and do not rewrite this target.

## Source budget

| Crate | Sub-mechanism | Lines |
|---|---|---:|
| `glyphweave-sfnt` | scalar types, bounded reads, Byte Span arithmetic, Diagnostics | 520 |
| | Font Collection and Face selection | 420 |
| | Table Directory parsing, ordering, overlap, and checksum validation | 620 |
| | `cmap` formats 4, 12, and 14 plus Character Map resolution | 960 |
| | header, profile, metrics, and naming scalar tables | 480 |
| | table access and diagnostic rendering support | 300 |
| | **crate subtotal** | **3,300** |
| `glyphweave-outline` | Loca and Glyf boundary decoding | 720 |
| | Simple Glyph flags, coordinate deltas, contours, and instructions | 780 |
| | Composite Glyph components and transforms | 640 |
| | Glyph Closure, cycle detection, and depth limits | 460 |
| | dense Remap and component reference rewriting | 360 |
| | outline encoding and Diagnostics | 340 |
| | **crate subtotal** | **3,300** |
| `glyphweave-layout` | Coverage, Class Definition, device, and variation-index structures | 850 |
| | Script List, language systems, and Feature List | 550 |
| | Lookup List, flags, mark filtering, and extension dispatch | 450 |
| | supported GSUB lookup decoding and rewriting | 950 |
| | supported GPOS lookup decoding and rewriting | 1,050 |
| | layout closure, pruning, reference repair, and Diagnostics | 450 |
| | **crate subtotal** | **4,300** |
| `glyphweave-subset` | request model and Codepoint/Glyph Set selection | 500 |
| | Character Map precedence and Variation Selector resolution | 420 |
| | Subset Plan validation and Table Pass scheduling | 520 |
| | cross-table Glyph Closure fixed point | 600 |
| | table Retention and declared opaque-table policy | 360 |
| | metadata, names, Unicode ranges, and profile transformations | 500 |
| | deterministic ordering and pipeline Diagnostics | 300 |
| | **crate subtotal** | **3,200** |
| `glyphweave-emit` | canonical primitive and record encoding | 420 |
| | SFNT directory layout, alignment, Checksum, and adjustment | 500 |
| | Character Map rebuilding | 620 |
| | Glyf and Loca rebuilding | 520 |
| | Horizontal Metric and profile rebuilding | 360 |
| | Layout Table rebuilding | 500 |
| | name, OS/2, post, and retained-table emission | 280 |
| | **crate subtotal** | **3,200** |
| `glyphweave-verify` | Table Directory and required-table Verification | 340 |
| | Byte Span, alignment, overlap, and Checksum Verification | 320 |
| | outline bounds, components, closure, and Remap Verification | 460 |
| | Character Map semantic Verification | 360 |
| | layout reference and glyph-bound Verification | 430 |
| | round-trip properties and Shape Probe model | 400 |
| | reports and independent Diagnostics | 290 |
| | **crate subtotal** | **2,600** |
| `glyphweave-cli` | arguments, response files, and help | 320 |
| | Codepoint Set, script, language, and feature selection syntax | 260 |
| | bounded file reads and atomic artifact writes | 280 |
| | human and machine-readable Diagnostic output | 220 |
| | Verification and differential-reference modes | 220 |
| | exit behavior and shell completion | 100 |
| | **crate subtotal** | **1,400** |
| | **TOTAL SOURCE** | **21,300** |

## Test budget

| Crate | Primary technique | Test lines |
|---|---|---:|
| `glyphweave-sfnt` | conformance fixtures, parser properties, fuzz regressions | 3,300 |
| `glyphweave-outline` | generated glyph graphs, round trips, closure properties | 3,600 |
| `glyphweave-layout` | golden layouts, reference comparisons, remap properties | 4,300 |
| `glyphweave-subset` | pass-sequence properties and differential subset manifests | 3,300 |
| `glyphweave-emit` | byte-level goldens, checksum properties, reproducibility | 2,700 |
| `glyphweave-verify` | fault injection and emitter mutation fixtures | 1,900 |
| `glyphweave-cli` | built-binary end-to-end scenarios | 1,200 |
| **TOTAL TESTS** | | **20,300** |

Test-to-source ratio: **0.953**.

## Structural budget

- 7 crates.
- 110–125 source files, averaging 170–195 lines.
- No source file over 500 lines.
- At least 650 focused tests by 1.0.
- No more than 26 direct third-party dependencies.
- No more than 115 public API items.


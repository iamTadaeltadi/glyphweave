# Glyphweave

Glyphweave is a deterministic, memory-safe OpenType/TrueType font-subsetting compiler written in Rust. Given a font and a Unicode selection, it will retain required glyphs and layout behavior, emit a smaller valid SFNT font, and independently verify the artifact.

The repository is at M0. The frozen domain and invariants are in `PROJECT.md`, crate boundaries are in `ARCHITECTURE.md`, the honest 21.3K-line decomposition is in `BUDGET.md`, and the sole next task is marked in `PROGRESS.md`.

## Development

Rust 1.85.0 or newer and `just` are required.

```sh
just check
```

This runs formatting verification, Clippy with warnings denied, all workspace tests, and rustdoc with warnings denied.

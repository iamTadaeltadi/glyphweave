# ADR 0007: Run property tests with fixed seeds

- Status: Accepted
- Date: 2026-07-26

## Context

M1 has several invariants over large domains: Unicode scalar membership, checked byte ranges, glyph
sets, dense remaps, canonical plans, and immutable artifacts. Hand-selected examples cover known
boundaries but do not systematically compose values or shrink a failing case.

The computational core forbids randomness, clocks, environment access, and I/O. Tests may generate
values, but a quality gate that depends on ambient entropy can fail irreproducibly. Shared property
support also cannot justify an eighth production crate because the frozen architecture and
structural budget specify seven crates.

## Decision

Use `proptest` 1.11 as a test-only dependency. It supports the workspace MSRV of Rust 1.85 and
provides composable strategies plus automatic shrinking.

Keep generators and invariant oracles inside the tests of the crate that owns each domain type.
Cross-crate properties live in the highest crate whose public API can observe the complete
relationship. Generator code never enters a production library artifact.

Every suite constructs a `TestRunner` with an explicit fixed seed, an explicit case count, and no
filesystem failure persistence. Different suites use different committed seeds. A newly discovered
minimal counterexample becomes a named regression test before its generator changes.

Properties must assert domain laws rather than duplicate implementation steps. M1 suites cover at
least checked-span bounds, canonical set/map ordering, `.notdef` retention, Remap totality,
injectivity and density, plan determinism, and immutable artifact ownership.

## Consequences

- The same revision explores the same cases on every supported operating system.
- Failures shrink to reviewable values and can be promoted to permanent regression tests.
- Property machinery adds no runtime dependency and cannot leak entropy into font transformation.
- Each crate owns the smallest strategies needed to express its public invariants.
- Fixed suites do not replace fuzzing; M7 fuzz targets will explore unbounded evolving inputs.

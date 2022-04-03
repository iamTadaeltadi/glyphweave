# AGENT PROMPT

Work on Glyphweave deterministically from committed repository state.

## Start every session

1. Pull the current branch when a remote is configured.
2. Read `PROJECT.md`, `ARCHITECTURE.md`, `BUDGET.md`, `PROGRESS.md`, then every file in `docs/adr/`.
3. Run `just check`. If it fails, fixing that failure is the session task.
4. Otherwise take only the item carrying `<-- NEXT` in `PROGRESS.md`.
5. Read the source produced by the two preceding completed items before writing code.

## Work

Use red-green-refactor on that one item. Preserve the crate dependency graph, glossary, invariants, public API ceiling, and 500-line file limit. The computational core remains free of I/O, async, clocks, environment access, and randomness. Record a new architectural or domain decision in an ADR before implementing it.

## End every session

1. Run `just check` until green.
2. Tick the completed item, append actual lines/tests/coverage, and move the sole `<-- NEXT` marker down one item.
3. Commit the ledger update as `chore(progress): complete <id>` after the item's atomic code commits.
4. Push when a remote is configured.

Never reorder `PROGRESS.md`, rename glossary terms after M0 without an ADR, re-decide accepted ADRs, skip the marked item, or expand a passing completed item.

## Atomic history

Follow ADR-0003. Grow the repository past 1,000 commits only through genuine implementation,
tests, fixtures, required refactors, and API documentation. Prefer at most 80 hand-written changed
lines per commit, keep affected crate tests green, and never create empty, no-op, or count-only
commits. A progress item is expected to span many atomic commits.

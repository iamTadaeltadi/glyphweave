# ADR 0003: Grow the repository through genuine atomic commits

- Status: Accepted
- Date: 2026-07-26

## Context

Glyphweave targets more than 1,000 commits by 1.0. That history is useful only when each commit
captures a real, reviewable step in the font-subsetting implementation. Empty commits, no-op
changes, repeated formatting churn, and artificial after-the-fact splitting would inflate the
count without preserving engineering decisions.

The M0 and M1.1 commits are already published. Rewriting them would invalidate the public tag and
remote branch, so the atomic policy applies prospectively from M1.2.

## Decision

Reach more than 1,000 commits through real implementation work. A normal commit introduces one
cohesive unit: a typed invariant, one decoding rule, a Diagnostic cause, a checksum rule, a test
fixture, a focused refactor required by current work, or documentation for an implemented API.

Code commits must leave their affected crate tests green. Each progress item ends with `just check`
and a separate progress-ledger commit. The preferred change size is at most 80 hand-written lines;
larger changes require an indivisible generated fixture, snapshot, or mechanical move and explain
that exception in the commit message.

No commit may exist solely to increase the count. Commit timestamps and ancestry must reflect the
actual development sequence. Progress items may span many commits; the final commit for an item is
`chore(progress): complete <id>`.

## Consequences

- Reviews and regressions can identify the exact rule that changed.
- The repository reaches the target gradually across the planned 21,300 source and 20,300 test
  lines instead of manufacturing history at release time.
- Some red-green-refactor cycles keep a test and its minimal implementation together so every
  published commit remains buildable.
- Existing published commits remain unchanged.

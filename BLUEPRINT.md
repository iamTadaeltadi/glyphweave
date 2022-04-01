# BLUEPRINT — REUSABLE ACROSS REPOS

Drop this in the repo root next to `AGENT_PROMPT.md`. It is domain-agnostic. Section 1 is the only part you edit per repo; everything else is fixed.

Two problems this solves:
- **Session drift** — a new session must continue exactly where the last stopped, not re-decide things. Solved in §6 by keeping all state in committed files, never in the model's memory.
- **Repo sameness** — the same prompt across ten repos produces ten identical-looking codebases. Solved in §5 by making the domain drive architecture and naming, instead of defaulting to one template.

---

## 1. THE DOMAIN CARD

Fill this once per repo. Commit it as `PROJECT.md`. **Frozen after M0** — changing it later requires an ADR.

```markdown
# PROJECT

Name:          <name>
One sentence:  <what it does, for whom>
Domain shape:  <one of the seven in §2>
Architecture:  <derived from shape, §2>
Source target: 20 000 – 25 000 lines
Test target:   ≥ 0.9 × source lines
Crates:        <5–9, listed with one-line responsibility each>
MSRV:          <pinned>

## Glossary
<15–40 domain terms with definitions. WRITTEN BEFORE ANY CODE.>
<Every type, function, and module name in this repo comes from this list.>

## Irreducible capabilities
<8–15 things the system must do. Not features — capabilities.>

## Invariants
<5–10 rules that must never be violated. Each becomes a type or a property test.>

## Reference implementations
<what exists publicly, and how this differs. Drives differential testing.>

## Non-goals
<explicitly out of scope, so no session drifts into building it>
```

---

## 2. ARCHITECTURE SELECTION — DO NOT DEFAULT

Hexagonal `domain/application/adapters` is not universal. Forcing it onto a compiler produces absurd code. Pick by shape:

| Domain shape | Architecture | Core structure | Examples |
|---|---|---|---|
| **Transformation pipeline** | Pass-based | `Stage<In, Out>` chain, shared context, span-carrying diagnostics | compiler, transpiler, query planner, image pipeline |
| **Layered resource manager** | Strict layering | each layer only calls the one below; no upward calls | LSM engine, filesystem, cache, allocator |
| **Replicated state machine** | Pure FSM + ports | `step(state, event) -> (state, effects)`; I/O outside the core | Raft, CRDT sync, gossip, consensus |
| **Content-addressed store** | Object model + command layer | immutable objects, refs, plumbing/porcelain split | git-like, package manager, artifact store |
| **Request/response service** | Hexagonal | domain / application / adapters | API backend, scheduler, job runner |
| **Protocol implementation** | Codec + state machine + transport | sans-I/O core, transport injected | HTTP/2, QUIC, Kafka client, MQTT |
| **Interactive editor** | Model + command + undo | commands are values; undo is free | editor, DAW, CAD, terminal UI |

**Rule:** the crate names come from the architecture, which comes from the shape. A pipeline repo has crates named `lexer/parser/typeck/codegen`, not `domain/application/adapters`. Using the wrong skeleton is the single biggest cause of repos looking identical.

**Universal rule that survives all shapes:** the computational core has zero I/O, zero async, zero clock, zero randomness. Everything nondeterministic is injected. That is what makes any of these testable.

---

## 3. DERIVING 20–25K LINES HONESTLY

Do this arithmetic **before** writing code, and commit it as `BUDGET.md`.

1. List the irreducible capabilities from the domain card.
2. Decompose each into sub-mechanisms.
3. Budget each sub-mechanism in lines.
4. Sum.

**If the sum is under 20k, the domain is too small.** Add *depth*, never breadth:

| Add depth (real lines) | Never add breadth (filler) |
|---|---|
| more formats, encodings, versions | config systems for constants that never change |
| more optimization passes | wrapper crates, re-export modules |
| real error recovery, not just detection | getters/setters, ceremonial traits |
| edge cases the reference impl handles | extra layers of indirection |
| a second backend / storage format | CRUD endpoints nobody calls |
| observability, admin, repair tooling | speculative plugin systems |

If depth still doesn't reach 20k, **pick a different domain**. Do not pad. A tight 9k-line repo is worth more than a padded 22k one, and padding is visible in about thirty seconds of reading.

**Sanity ratios** at 20–25k source:
- 5–9 crates · 90–160 source files · avg file ~160 lines, none over 500
- 500–900 tests · test lines ≥ 0.9 × source lines
- ≤ 30 direct dependencies · ≤ 120 public API items
- 1 000–1 800 commits

---

## 4. TEST STRATEGY BY SHAPE

At this scale you do not reach 80% with hand-written unit tests. You reach it by picking the high-leverage technique for your shape:

| Shape | Primary technique | Gets you |
|---|---|---|
| Pipeline | golden/snapshot tests (`insta`) + fuzz the front end (`cargo-fuzz`) + differential vs reference | huge coverage per test line |
| Layered storage | model-based property tests vs `BTreeMap` + crash injection at every fsync | correctness under failure |
| Replicated FSM | deterministic simulation testing — seeded clock, network, faults; replay any failure by seed | the only way to test distributed code |
| Content store | differential vs the real tool + round-trip properties | catches spec deviations free |
| Service | contract tests at each port + `testcontainers` integration | real adapters, fast core |
| Protocol | published conformance vectors + fuzz the decoder + interop vs a real peer | spec compliance |
| Editor | command-sequence property tests: apply N random commands, undo N, assert identity | invariant coverage |

Fixed floors regardless of shape: **line ≥ 80%, branch ≥ 70%**, computational core **≥ 90%**. Every error variant produced by a test. Every public item doctested. Then `cargo mutants` — survivors killed or justified in writing.

---

## 5. ANTI-REPETITION — SO TEN REPOS DON'T LOOK LIKE ONE

Enforce all six. This is what separates ten real projects from one template printed ten times.

1. **Glossary-first naming.** Every identifier comes from `PROJECT.md`'s glossary. A storage engine has `Memtable`, `Sstable`, `Manifest`, `Tombstone`. A compiler has `Span`, `Binding`, `Substitution`, `Lowering`. If two of your repos share a type name that isn't a stdlib concept, one of them is misnamed.

2. **Banned in every repo, forever:** `Manager`, `Handler`, `Helper`, `Util(s)`, `Common`, `Base`, `Data`, `Info`, `Service`, `Processor`, `Engine` (unless the domain genuinely uses the word), `handle_*`, `process_*`, `do_*`.

3. **Architecture from §2, never defaulted.** Different shape → different crate names, different module tree, different core abstraction.

4. **Minimum three domain-specific typed invariants per repo** that no other repo of yours has. A typestate for a protocol handshake. A phantom-typed `Lsn` that can't be compared across generations. A `NonEmptyPath` that can't be constructed empty. These are the fingerprint of real modeling.

5. **Different test technique per §4.** Repos that all use plain unit tests read as one author with one habit. Repos that use simulation, differential, golden, and property testing respectively read as four real projects.

6. **Different error taxonomy per repo.** Error enums should describe *this* domain's failures. If two repos share an error enum shape, you copied instead of modeled.

---

## 6. CROSS-SESSION DETERMINISM

A new session has no memory. It must reconstruct state from the repo alone. That works only if state lives in committed files.

### Committed state files

| File | Role | Mutability |
|---|---|---|
| `PROJECT.md` | domain card, glossary, invariants | frozen after M0 |
| `BUDGET.md` | per-module line budget from §3 | frozen; deviations noted, not edited |
| `PROGRESS.md` | ordered checklist, single source of truth for "what's next" | append/tick only — **never reorder** |
| `docs/adr/NNNN-*.md` | every decision, with rejected options | append-only, never edited |
| `ARCHITECTURE.md` | crate graph, dependency rule, where things go | edit only via ADR |

### `PROGRESS.md` format

```markdown
# PROGRESS

## M1 — <module>  (budget: 3 200 lines)
- [x] 1.1 <capability>            — 412 lines, 18 tests, 94% cov
- [x] 1.2 <capability>            — 380 lines, 22 tests, 91% cov
- [ ] 1.3 <capability>            <-- NEXT
- [ ] 1.4 <capability>
```

Exactly one `<-- NEXT` marker exists in the file at any time.

### Session protocol — mandatory, every session

```
START
 1. git pull
 2. Read in order: PROJECT.md, ARCHITECTURE.md, BUDGET.md, PROGRESS.md, docs/adr/*
 3. Run `just check`. If red, fixing it IS this session's task. Stop here.
 4. Take the item marked <-- NEXT. Do not choose. Do not skip. Do not reorder.
 5. Read the source of the two items completed before it, to match style.

WORK
 6. Red-green-refactor on that item only. No scope creep.
 7. Any decision made → write an ADR before continuing.

END
 8. `just check` green.
 9. Tick the box, record actual lines/tests/coverage, move <-- NEXT down one.
10. Commit: `chore(progress): complete <id>`
11. Push.
```

**Forbidden in every session:** re-deciding anything in an ADR, renaming anything in the glossary, reordering `PROGRESS.md`, starting an item that isn't `<-- NEXT`, "improving" a completed item that passes the gate.

That last set of prohibitions is what makes session 40 behave like session 1.

### Style anchoring

Committed decisions beat remembered ones. Because `rustfmt.toml`, `clippy.toml`, the workspace lint table, and the glossary are all in the repo, and because step 5 makes each session read the two preceding items, style converges instead of drifting. There is nothing left for a session to invent.

---

## 7. MILESTONES AT THIS SCALE

| M | Content | Source | Exit |
|---|---|---|---|
| M0 | Workspace, all tooling configs, CI green, `PROJECT.md`, `BUDGET.md`, `PROGRESS.md`, ADR-0001 | ~300 | CI green on 3 OSes, tag `v0.0.1` |
| M1 | Core data model + invariants as types. No I/O. | 3–5k | core coverage ≥ 90% |
| M2 | Primary algorithm / main mechanism | 4–6k | property tests passing |
| M3 | Persistence or transport | 3–4k | crash/fault injection passing |
| M4 | Second major subsystem (per shape) | 4–6k | differential tests vs reference |
| M5 | Surface: CLI / API / REPL | 2–3k | E2E against the built binary |
| M6 | Observability, admin, repair tooling | 1–2k | metrics + structured tracing |
| M7 | Hardening: fuzz, mutants, bench, docs, release | 1–2k | all §17 conditions, tag `v1.0.0` |

Re-run the full self-critique across the **entire** codebase at every milestone boundary, not just on new code.

---

## 8. PRE-FILLED DOMAIN CARDS

Six ready to use. Each is a distinct shape, so six repos built from these will not resemble each other.

| Project | Shape | Architecture | Core invariants to encode | Test technique | Source |
|---|---|---|---|---|---|
| **LSM storage engine** | Layered | strict layering | LSN monotonicity, level-size ratios, tombstone survives until bottom level | model-based vs `BTreeMap` + crash injection | ~21k |
| **Language + compiler** | Pipeline | pass-based | typed-AST unforgeable, every node has a `Span`, resolution before typecheck | golden tests + fuzz + differential | ~25k |
| **Raft KV** | Replicated FSM | pure FSM + ports | term monotonicity, log matching, commit-index safety | deterministic simulation, seeded | ~20k |
| **Git implementation** | Content store | object model + commands | object IDs immutable, packfile delta chains acyclic, refs never dangle | differential vs real `git` | ~22k |
| **QUIC / HTTP-2 stack** | Protocol | sans-I/O codec + FSM | stream-ID parity, flow-control never negative, frame ordering | conformance vectors + fuzz + interop | ~23k |
| **Structured editor** | Editor | model + command + undo | rope indices always valid, undo stack invertible, cursor within bounds | command-sequence property tests | ~20k |

---

## 9. BOOTSTRAP SEQUENCE FOR A NEW REPO

```
1. Copy AGENT_PROMPT.md + BLUEPRINT.md into the new repo.
2. Fill §1 domain card → PROJECT.md.  Glossary FIRST.  No code yet.
3. Pick shape from §2 → architecture → crate names.
4. Do §3 arithmetic → BUDGET.md.  If under 20k, change domain now, not later.
5. Expand budget into an ordered checklist → PROGRESS.md, one <-- NEXT.
6. M0: scaffold, CI, push, wait for green, tag v0.0.1.
7. Every session thereafter: run the §6 protocol. Nothing else.
```

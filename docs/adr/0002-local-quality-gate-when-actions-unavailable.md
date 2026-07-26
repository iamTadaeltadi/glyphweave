# ADR 0002: Treat the local MSRV quality gate as authoritative when GitHub Actions is unavailable

- Status: Accepted
- Date: 2026-07-26

## Context

The M0 exit criterion asked for a three-operating-system continuous-integration run to
pass before tagging `v0.0.1`. The `iamTadaeltadi/glyphweave` repository is private, which
is a fixed requirement, and private repositories consume metered Actions minutes. On the
first push, every matrix job (`ubuntu-latest`, `macos-latest`, `windows-latest`) was
refused before starting with an account-level billing error and never executed any step.
This is outside the repository and cannot be resolved from the codebase.

The workflow itself is correct: `.github/workflows/ci.yml` runs the same four commands the
local gate runs — `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets
--all-features -- -D warnings`, `cargo test --workspace --all-targets --all-features`, and
`cargo doc --workspace --no-deps --all-features` — pinned to Rust 1.85.0. Locally, `just
check` runs that identical set on the pinned toolchain and is green.

## Decision

Keep `ci.yml` committed and unchanged. When GitHub Actions cannot run because of the
account billing block, the local `just check` gate on the pinned 1.85.0 toolchain is the
authoritative pre-tag gate. Tag `v0.0.1` on a green local gate. When Actions billing is
restored, the existing workflow runs on the next push with no repository change required;
remote three-OS confirmation is deferred, not removed.

## Rejected options

- **Make the repository public for free Actions minutes:** rejected. The repository must
  stay private; that requirement outranks CI convenience.
- **Delete or disable the workflow:** rejected. The workflow is correct and should run
  automatically once billing is restored; removing it would hide the intended gate.
- **Block all further work until billing is fixed:** rejected. The commands the workflow
  runs already pass locally on the pinned toolchain, and the computational core is pure
  Rust with no platform-specific code, so the marginal risk of a three-OS gap is low.
- **Self-hosted runner:** rejected as disproportionate for a pure-Rust library at this
  stage.

## Consequences

- Progress can continue on M1 with the same command-level guarantees CI would provide,
  minus operating-system breadth.
- Platform-specific defects (path handling, line endings, integer widths) would not be
  caught until Actions runs; the pure, `std`-only core makes this unlikely but not
  impossible, and later I/O work in `glyphweave-cli` must be reviewed with that in mind.
- No code or workflow change is needed to restore full CI; only the account billing state
  must change.

set shell := ["sh", "-eu", "-c"]

msrv := "1.85.0"

default: check

fmt:
    cargo +{{msrv}} fmt --all -- --check

clippy:
    cargo +{{msrv}} clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo +{{msrv}} test --workspace --all-targets --all-features

doc:
    RUSTDOCFLAGS="-D warnings" cargo +{{msrv}} doc --workspace --no-deps --all-features

check: fmt clippy test doc


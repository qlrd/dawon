default: check

# Run ganesha pre-commit hooks that apply to a Rust project.
# Skips norminette, compiler, forbidden-functions (C-only).
# Only commit-message is wired in .pre-commit-config.yaml.
pcc:
    pre-commit run --all-files

build:
    cargo build

release:
    cargo build --release

test:
    cargo test

test-python:
    cd tests/python && uv run pytest -q

test-all: test test-python

fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

clippy:
    cargo clippy -- -D warnings

check: fmt-check clippy test test-python

coverage:
    cargo llvm-cov --summary-only

doc:
    cargo doc --no-deps --document-private-items --open

fuzz-forbidden:
    cargo +nightly fuzz run fuzz_forbidden

fuzz-config:
    cargo +nightly fuzz run fuzz_config

clean:
    cargo clean

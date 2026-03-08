# CLAUDE.md

Instructions for Claude Code when working in this repository.

## Build

```bash
just build          # cargo build
just release        # cargo build --release
```

## Test

```bash
just test           # cargo test
just test-python    # cd tests/python && uv run pytest -q
just test-all       # both
```

## Full CI check (run before every commit)

```bash
just check          # fmt-check + clippy + test + test-python
```

## Fuzz

Fuzz targets require the nightly toolchain:

```bash
just fuzz-forbidden
just fuzz-config
```

## Pre-commit (ganesha)

```bash
just pcc            # pre-commit run --all-files
```

Only the `commit-message` hook is active for this Rust project
(`.pre-commit-config.yaml`).  C-only hooks (norminette, compiler,
forbidden-functions) are not wired in.

## Other recipes

```bash
just fmt            # cargo fmt
just fmt-check      # cargo fmt --check
just clippy         # cargo clippy -- -D warnings
just doc            # cargo doc --open
just clean          # cargo clean
```

## Project layout

```
src/
├── lib.rs               public API
├── main.rs              CLI thin wrapper
├── cli.rs               clap CLI definitions
├── config.rs            .gato.toml loader
├── eval.rs              per-exercise orchestration
├── report.rs            output formatting
├── checks/
│   ├── mod.rs
│   ├── norminette.rs    subprocess: norminette
│   ├── compiler.rs      subprocess: cc + ASAN/UBSAN
│   ├── forbidden.rs     regex scan + nm symbol table
│   ├── harness.rs       fork+pipe SHA-256 C harness
│   └── valgrind.rs      subprocess: valgrind
└── subjects/
    ├── mod.rs
    ├── c00.rs           C00 exercise definitions + test vectors
    └── c01.rs … c09.rs  C01–C09 exercise definitions

tests/
├── test_harness.rs      integration: harness round-trip
├── test_config.rs       integration: config loading
├── test_forbidden.rs    integration: forbidden-function scan
└── python/
    ├── conftest.py      fixtures (binary path, run, module_dir)
    ├── test_cli.py      CLI functional tests
    ├── test_checks.py   check-level functional tests
    └── fixtures/
        ├── clean/       valid C files
        ├── forbidden/   files using forbidden functions
        └── norm_error/  norminette-failing files

fuzz/
└── fuzz_targets/
    ├── fuzz_forbidden.rs
    └── fuzz_config.rs
```

## Conventions

- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings` — zero warnings allowed
- New checks: add `src/checks/<name>.rs`, export in
  `src/checks/mod.rs`, wire in `src/eval.rs`
- New subjects: add `src/subjects/<module>.rs`, export in
  `src/subjects/mod.rs`, add test vectors with strict edge cases
- Commit messages follow Conventional Commits 1.0.0,
  subject line must not exceed 72 characters
- Prose (comments, docs, Markdown) wraps at 72 characters
- No unnecessary dependencies — prefer `std` where possible

## Commit message format

```
<type>[(<scope>)][!]: <description>
```

Types: `feat` `fix` `docs` `style` `refactor` `perf` `test`
`build` `ci` `chore` `revert` `init`

## Environment

Set the git editor to vim:

```bash
git config --global core.editor vim
```

## Key design decisions

- ASAN/UBSAN enabled at compile time in the C test harness.
- Forbidden check has two layers: regex scan then `nm -u` symbol
  table — catches indirect calls that bypass source-level analysis.
- Harness uses `dawon_capture` (fork+pipe binary protocol): each
  test's stdout is SHA-256 hashed in Rust and compared against the
  commitment in `TestCase.expected_sha256`. No expected bytes are
  embedded in the generated C source.
- `libloading` verifies symbol presence in the student's `.so`
  before running harness tests.
- `cc -Wall -Wextra -Werror -fsyntax-only` for syntax-only check;
  ASAN build uses a separate temp directory.
- `.gato.toml` absent → `Config::default()` — student is never
  blocked by a missing config file.
- `cargo-fuzz` invariant: `ft_` prefixed names must never appear
  as forbidden-function violations.

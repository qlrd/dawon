# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Fixed

- Harness: internal names renamed from `gato` to `dawon`
  throughout the generated C source.
- Harness: `ft_is_negative` SHA-256 commitments corrected for
  `INT_MIN` and `0`.
- Harness: round-trip integration tests guarded with
  `#[cfg(unix)]` — `sys/wait.h` is POSIX-only; Windows CI no
  longer fails to compile.
- Harness: removed `detect_leaks=1` from `ASAN_OPTIONS` — LSan
  is not available on macOS; macOS CI no longer aborts before
  writing any output.
- Harness: `compare_outputs` now `break`s on a truncated
  protocol frame — avoids misleading per-test errors after
  stream desync.
- Harness: drain pipe in a loop and close read-end before
  `waitpid` — prevents deadlock when child stdout fills the
  pipe buffer.
- Harness: `detect_leaks=1` restored on Linux via
  `cfg!(target_os = "linux")` — LSan coverage preserved while
  macOS stays clean.
- Config: `.gato.toml` renamed to `.dawon.toml` throughout
  docs and examples.
- Docs: wording, SHA-256 integrity description, and hash
  values corrected in README.

### Added

- CLI: `--rush` flag for `check`/`friend` to evaluate Rush00
  subjects.
- Subjects: initial Rush00 support (`Rush00/main.c`) with shared
  static checks (norminette, forbidden scan, compiler, valgrind).

- CI: ubuntu-latest, macos-latest, debian-stable (stable +
  MSRV 1.85); Valgrind gated to Linux.
- CI: pinned toolchain SHA, Cargo registry cache, `uv` for
  Python test dependencies.
- Docs: `.github/copilot-instructions.md` — repository-level
  Copilot custom instructions mirroring `CLAUDE.md`.
- Docs: atomic-commits policy in `CONTRIBUTING.md` —
  `git rebase -i --exec 'just check' main` recipe.
- Docs: Rust/Python formatting guidelines and issue-labelling
  workflow in `CLAUDE.md` and `CONTRIBUTING.md`.
- Lore: `assets/mascot.svg` and Mascot section in README.
- Python conftest: fallback from debug to release build when
  `target/debug/dawon` is absent.

---

## [0.1.0] — 2026-03-08

### Added

- `check` subcommand: evaluate one or all C00 exercises from a
  module directory.
- `friend` subcommand: evaluate a peer's submission by login or
  path.
- Six-layer check pipeline per exercise:
  1. Norminette subprocess (`norminette`).
  2. Symbol verification via `libloading` (student `.so`).
  3. Forbidden-function scan: regex `\b(func)\s*\(` + `nm -u`
     symbol table — catches indirect calls.
  4. Compiler check: `cc -Wall -Wextra -Werror -fsyntax-only`.
  5. ASAN/UBSAN harness: fork+pipe binary protocol; Rust reads
     captured stdout, SHA-256 hashes it, compares against stored
     commitments. No expected bytes in generated C.
  6. Valgrind: `valgrind --leak-check=full --error-exitcode=1`.
- C00 subject definitions (ex00–ex08) with strict test vectors:
  - `ft_print_comb`: C(10,3) = 120 combinations.
  - `ft_print_comb2`: C(100,2) = 4950 pairs.
  - `ft_is_negative`: `INT_MIN`, 0, positive, negative.
  - `ft_putchar`: null byte (`'\0'`), DEL (127).
- SHA-256 commitment model: expected outputs are stored as
  32-byte hashes — no plaintext answers in the source tree.
- `.dawon.toml` configuration: optional forbidden-function list;
  absent file returns safe defaults.
- GRAND SUMMARY section printed after all exercises.
- `--no-sanitizers` and `--no-valgrind` flags for environments
  where those tools are unavailable.
- Integration test suite (Rust): config, forbidden, harness
  round-trip (correct + incorrect `ft_putchar`).
- Python functional tests (pytest ≥ 8): CLI output shape, exit
  codes, `--no-sanitizers`, `--no-valgrind`, banner, grand
  summary.
- Fuzz targets (`cargo-fuzz`): `fuzz_forbidden`, `fuzz_config`.
- `CONTRIBUTING.md`, `SECURITY.md`, `CLAUDE.md`.

[Unreleased]: https://github.com/qlrd/dawon/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/qlrd/dawon/releases/tag/v0.1.0

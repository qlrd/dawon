# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.0] — 2026-03-07

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
  5. ASAN/UBSAN harness: fork+pipe byte-exact C test runner with
     `memcmp` on stdout; includes null byte and DEL (0x7F) cases.
  6. Valgrind: `valgrind --leak-check=full --error-exitcode=1`.
- C00 subject definitions (ex00–ex08) with strict test vectors.
  Pre-computed expected outputs via `const fn`:
  - `ft_print_comb`: C(10,3) = 120 combinations.
  - `ft_print_comb2`: C(100,2) = 4950 pairs.
- `.gato.toml` configuration: optional forbidden-function list;
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

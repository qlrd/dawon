# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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
  - `ft_print_combn`: generalized for n=1,2,3.
  - `ft_is_negative`: `INT_MIN`, 0, positive, negative.
  - `ft_putchar`: null byte (`'\0'`), DEL (127).
- C01–C09 subject definitions: pointer exercises, string
  manipulation, recursion, dynamic allocation, libft subset.
- SHA-256 commitment model: expected outputs are stored as
  32-byte hashes — no plaintext answers in the source tree.
- `.gato.toml` configuration: optional forbidden-function list;
  absent file returns safe defaults.
- GRAND SUMMARY section printed after all exercises.
- `--no-sanitizers` and `--no-valgrind` flags for environments
  where those tools are unavailable.
- Integration test suite (Rust): config, forbidden, harness
  round-trip (correct + incorrect `ft_gato_probe`).
- Python functional tests (pytest ≥ 8): CLI output shape, exit
  codes, `--no-sanitizers`, `--no-valgrind`, banner, grand
  summary.
- Fuzz targets (`cargo-fuzz`): `fuzz_forbidden`, `fuzz_config`.
- `CONTRIBUTING.md`, `SECURITY.md`, `CLAUDE.md`.

[Unreleased]: https://github.com/qlrd/dawon/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/qlrd/dawon/releases/tag/v0.1.0

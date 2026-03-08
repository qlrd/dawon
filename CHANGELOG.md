# Changelog

All notable changes to **dawon** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `CLAUDE.md` — guidance for AI assistants working in this repository.
- `CHANGELOG.md` — this file, tracking version history.
- `CONTRIBUTING.md` — guidelines for contributors.
- `LICENSE` — MIT license.

## [0.1.0] - 2026-03-07

### Added

- Rust evaluation core with six-layer check pipeline.
- SHA-256 commitment-based harness (no plaintext expected bytes in source).
- Check integrations including norminette, valgrind, and ASAN/UBSAN flows.
- Subject and evaluation orchestration under `src/subjects/` and `src/eval.rs`.
- Rust and Python functional test coverage.
- CI workflows and fuzz targets (`cargo-fuzz`) for robustness checks.

[Unreleased]: https://github.com/qlrd/dawon/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/qlrd/dawon/releases/tag/v0.1.0

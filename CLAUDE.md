# CLAUDE.md

This file provides guidance to Claude (and other AI assistants) when working with the **dawon** repository.

## Project Overview

**dawon** is a Rust-based super mini-moulinette. It evaluates subjects through a six-layer check pipeline and enforces strict invariants for deterministic grading.

## Repository Structure

```
dawon/
├── src/
│   ├── checks/        # Check implementations used by the pipeline
│   ├── subjects/      # Subject definitions and metadata
│   └── eval.rs        # Evaluation orchestration
├── tests/
│   └── python/        # Functional test suite
├── fuzz/              # cargo-fuzz targets
├── README.md
├── CLAUDE.md
├── CHANGELOG.md
├── CONTRIBUTING.md
└── LICENSE
```

## Development Guidelines

### Code Style

- Keep the codebase minimal and focused.
- Prefer clarity over cleverness.
- Follow the existing conventions already present in the code.

### Core Invariants

- **SHA-256 commitment invariant**: expected output bytes must never appear in plaintext in the source tree.
- Keep commitments and verification logic consistent whenever subjects/checks change.

### Testing

- Run `just check` before every commit.
- Run Rust and Python test suites for impacted areas.
- Run fuzz targets with `cargo fuzz` when changing parsing/evaluation logic.

### Commits

- Follow **Conventional Commits 1.0.0**.
- Keep commits small and focused on a single change.
- Reference related issues where applicable (e.g., `Closes #42`).

## Common Tasks

### Running the Project

Refer to [README.md](README.md) for build and run instructions.

### Making Changes

1. Fork the repository and create a feature branch.
2. Make your changes following the guidelines above.
3. Run `just check` and relevant targeted tests.
4. Submit a pull request with a clear description of the changes.

## Notes for AI Assistants

- **Minimal changes**: Prefer surgical, targeted edits. Do not refactor code that is unrelated to the task at hand.
- **No new dependencies**: Avoid adding new libraries or dependencies unless absolutely necessary.
- **Respect existing conventions**: Match the style, naming, and patterns already present in the codebase.
- **Security**: Never commit secrets, credentials, or sensitive information.

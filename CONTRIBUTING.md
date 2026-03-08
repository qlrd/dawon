# Contributing to dawon

Thank you for your interest in contributing to **dawon**! Contributions of all kinds are welcome — bug reports, feature requests, documentation improvements, and code patches.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Submitting a Pull Request](#submitting-a-pull-request)
- [Development Guidelines](#development-guidelines)
- [Commit Message Conventions](#commit-message-conventions)

---

## Code of Conduct

By participating in this project, you agree to treat all contributors with respect and to maintain a welcoming and inclusive environment. Please be constructive and professional in all interactions.

---

## Getting Started

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/dawon.git
   cd dawon
   ```
3. Create a new **feature branch**:
   ```bash
   git checkout -b feature/my-new-feature
   ```
4. Make your changes, then **test** them thoroughly.
5. **Push** the branch to your fork and open a **Pull Request**.

---

## How to Contribute

### Reporting Bugs

If you find a bug, please open an [issue](https://github.com/qlrd/dawon/issues) and include:

- A clear, descriptive title.
- Steps to reproduce the problem.
- Expected vs. actual behaviour.
- Environment details (OS, language version, etc.).

### Suggesting Features

Open an [issue](https://github.com/qlrd/dawon/issues) with the label **enhancement** and describe:

- The problem you are trying to solve.
- Your proposed solution or feature.
- Any alternatives you have considered.

### Submitting a Pull Request

1. Ensure your branch is up to date with `main`:
   ```bash
   git fetch origin
   git rebase origin/main
   ```
2. Make sure all existing tests pass.
3. Add or update tests as necessary for your changes.
4. Open a Pull Request against the `main` branch with:
   - A clear title and description.
   - A reference to any related issues (e.g., `Closes #42`).

---

## Development Guidelines

- **Minimal changes**: Keep pull requests focused and small.
- **No unrelated refactoring**: Only change code that is directly related to your fix or feature.
- **Follow existing conventions**: Match the code style, naming, and patterns already present in the project.
- **Document your changes**: Update `README.md` and `CHANGELOG.md` where applicable.
- **Run checks before commit**: Run `just check` before every commit.

### Rust Code Conventions

- Prefer clear and explicit code over clever abstractions.
- Keep modules focused; place checks in `src/checks/` and subject metadata in `src/subjects/`.
- Return structured errors instead of panicking in normal control flow.
- Keep public-facing behavior deterministic so checker results are reproducible.

### Adding a New Subject

- Do not commit plaintext expected answers/output bytes.
- Keep subject verification compatible with the SHA-256 commitment invariant.

---

## Commit Message Conventions

This project uses **Conventional Commits 1.0.0**.

Examples:

```
feat(checks): add timeout guard for parser
fix(eval): handle empty input path
docs(contributing): clarify subject commitment rules
```

Reference issues when relevant:

```
Fix crash when input is empty (closes #15)
```

---

Thank you for helping make **dawon** better! 🎉

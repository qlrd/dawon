# CLAUDE.md

This file provides guidance to Claude (and other AI assistants) when working with the **dawon** repository.

## Project Overview

**dawon** is a super mini-moulinette — a strict code checker/linter that is stricter than moulinette. It is designed to enforce coding standards with minimal configuration.

## Repository Structure

```
dawon/
├── README.md          # Project overview and quick-start guide
├── CLAUDE.md          # This file — AI assistant guidance
├── CHANGELOG.md       # Version history and release notes
├── CONTRIBUTING.md    # How to contribute to the project
└── LICENSE            # MIT license
```

## Development Guidelines

### Code Style

- Keep the codebase minimal and focused.
- Prefer clarity over cleverness.
- Follow the existing conventions already present in the code.

### Testing

- Run all existing tests before submitting changes.
- Add tests for any new functionality.
- Ensure all tests pass before opening a pull request.

### Commits

- Write clear, concise commit messages in the imperative mood (e.g., "Add feature X", "Fix bug Y").
- Keep commits small and focused on a single change.
- Reference related issues in commit messages where applicable (e.g., `Closes #42`).

## Common Tasks

### Running the Project

Refer to [README.md](README.md) for build and run instructions.

### Making Changes

1. Fork the repository and create a feature branch.
2. Make your changes following the guidelines above.
3. Run tests to verify nothing is broken.
4. Submit a pull request with a clear description of the changes.

## Notes for AI Assistants

- **Minimal changes**: Prefer surgical, targeted edits. Do not refactor code that is unrelated to the task at hand.
- **No new dependencies**: Avoid adding new libraries or dependencies unless absolutely necessary.
- **Respect existing conventions**: Match the style, naming, and patterns already present in the codebase.
- **Security**: Never commit secrets, credentials, or sensitive information.

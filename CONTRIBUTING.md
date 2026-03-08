# Contributing

Thank you for contributing to Dawon.

---

## Before you start

**Always open an issue before writing code**, even for small changes.
This lets maintainers prioritise the backlog, avoid duplicate work,
and assign the right labels before anyone spends time on a PR.

The only exception is a one-line typo fix where the intent is
completely unambiguous.

---

## Issue labels

Use labels when opening an issue so it can be triaged and
prioritised immediately.

| Label | When to use |
|-------|-------------|
| `good first issue` | Self-contained, well-scoped, newcomer-friendly |
| `bug` | Something is broken or produces wrong output |
| `enhancement` | New feature or improvement to existing behaviour |
| `docs` | Documentation-only change |
| `refactor` | Internal restructuring, no behaviour change |
| `test` | Missing or inadequate test coverage |
| `ci` | Workflow, justfile, or toolchain change |
| `security` | Vulnerability or hardening concern |
| `help wanted` | Maintainer lacks bandwidth; contributions welcome |
| `question` | Discussion or clarification needed before work starts |
| `blocked` | Cannot proceed until another issue/PR is resolved |

**Priority is driven by labels.**  Issues tagged `good first issue`
are the recommended entry point for new contributors.  Issues tagged
`help wanted` are higher-priority and may already have a rough design
in comments — read them before starting.

To add or update labels, use the GitHub UI or:

```bash
gh issue edit <NUMBER> --add-label "good first issue"
```

---

## Getting started

Fork the repository, then clone your fork:

```bash
git clone https://github.com/YOUR_USERNAME/dawon
cd dawon
```

Build the project and run the full test suite:

```bash
just build
just test-all
```

All tests must pass before opening a pull request.

---

## Code style

- Format Rust code with `cargo fmt` before every commit.
- Lint with `cargo clippy -- -D warnings`. No warnings are allowed.
- Keep prose (comments, docs, Markdown) at or below 72 characters
  per line.
- Do not add unnecessary dependencies. Prefer `std` where possible.

---

## Commit messages

This project uses **Conventional Commits 1.0.0**.

```
<type>[(<scope>)][!]: <description>
```

- The subject line must not exceed 72 characters.
- Use the imperative mood: "add support" not "added support".
- Types: `feat` `fix` `docs` `style` `refactor` `perf` `test`
  `build` `ci` `chore` `revert` `init`
- Append `!` after the type/scope to signal a breaking change.
  Add a body explaining why.

Examples:

```
feat(harness): add INT_MIN edge case for ft_putnbr
fix(forbidden): handle multi-line macro definitions
docs: update configuration section in README.md
test: add round-trip for correct ft_print_comb
refactor(eval): extract per-exercise summary printer
feat!: change exit code from 2 to 3 on internal errors
```

Set your editor to vim so `git commit` opens correctly:

```bash
git config --global core.editor vim
```

---

## Developer Certificate of Origin

All contributions must include a `Signed-off-by` footer. Add it
automatically with:

```bash
git commit -s
```

By signing off you certify that you have the right to submit your
work under the terms of the MIT License. See the full
[DCO](https://developercertificate.org/).

---

## Adding a new check

1. Create `src/checks/new_check.rs` with a public `check` function
   returning `CheckResult`.
2. Export it in `src/checks/mod.rs`:

   ```rust
   pub mod new_check;
   ```

3. Wire it into the per-exercise pipeline in `src/eval.rs`.
4. Write unit tests inside the module (`#[cfg(test)]`) and an
   integration test file in `tests/test_new_check.rs`.
5. Update `CHANGELOG.md` under `[Unreleased]`.

---

## Adding a new subject (module)

1. Create `src/subjects/<module>.rs` with `Subject` definitions
   and `TestCase` vectors. Include strict edge cases: `INT_MIN`,
   null byte (`'\0'`), DEL (127), empty input.
   Use SHA-256 commitment hashes — no plaintext answers in source.
2. Export it in `src/subjects/mod.rs`.
3. Wire the new subject into `src/cli.rs` and `src/eval.rs`.
4. Add fixtures under `tests/python/fixtures/` as needed.
5. Update `CHANGELOG.md` under `[Unreleased]`.

---

## tACK format

When a PR has been tested locally, leave a review comment with:

```
tACK <sha>
Tested locally against branch tip:

    OS:     $(uname -sr)
    cc:     $(cc --version | head -1)
    rustc:  $(rustc --version)
    cargo:  $(cargo --version)
    Python: $(python --version)

    cargo fmt --check                    ✓
    cargo clippy -- -D warnings          ✓
    cargo test                           ✓
    cd tests/python && uv run pytest     ✓
```

If you test again after changes, use `re-tACK <sha>` with a note
on what changed between your test runs.

---

## Pull request size

Keep each PR small enough that a human can review it in one sitting.
Aim for **one concern per PR**.

| Rule | Rationale |
|------|-----------|
| ≤ 400 lines changed (excluding generated/fixture files) | Reviewers lose focus beyond that |
| One commit type per PR (e.g. `feat` or `docs`, not both) | Makes the diff predictable |
| Separate refactors from feature additions | Mixing intent hides bugs |
| Separate docs-only changes from code changes | Docs can be merged faster |
| Open an issue first for anything larger | Agree on approach before writing code |

If your change is genuinely large (new module, new check pipeline),
**split it into a stack of small PRs**, each green on CI:

1. PR 1 — skeleton / types only
2. PR 2 — implementation
3. PR 3 — tests
4. PR 4 — documentation

Each PR in the stack must be independently reviewable and buildable.

---

## Submitting a pull request

1. Create a branch from `main`:

   ```bash
   git checkout -b feat/my-feature
   ```

2. Make your changes and add or update tests as needed.
3. Ensure all CI checks pass locally:

   ```bash
   just check
   ```

4. Push your branch and open a pull request against `main`.
5. Describe what the PR does, why it is needed, and how it was
   tested. Reference any related issues.

Pull requests require at least one review from a maintainer before
merging. The CI pipeline must be green.

---

## Reporting bugs

Open an issue at:

```
https://github.com/qlrd/dawon/issues
```

Include:

- Your operating system and architecture
- Rust version (`rustc --version`)
- cc version (`cc --version | head -1`)
- Exact steps to reproduce the problem
- Expected behaviour vs. actual behaviour
- Any relevant output or error messages

---

## License

By contributing you agree that your work will be released under the
terms of the MIT License.

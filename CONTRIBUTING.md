# Contributing

Thank you for contributing to Dawon.

---

## Before you start

Open an issue to discuss significant changes before writing code.
For small fixes (typos, obvious bugs) a direct pull request is fine.

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

## Atomic commits

Every commit merged to `main` must pass CI on its own.  Do not
bundle a fix for a red CI with unrelated changes so that the bundle
passes while the individual commit would not.

Practically: before pushing, test each commit in isolation:

```bash
git stash        # stash uncommitted work
cargo fmt --check
cargo clippy -- -D warnings
cargo test
git stash pop    # restore
```

Or use `git rebase -i` to squash/fixup commits that only exist to
"fix the previous commit" before opening the PR.

The CI ruleset enforces linear history.  A commit that breaks
`main` in isolation cannot be fixed by the next commit — it must
be amended before merge.

---

## Submitting a pull request

1. Create a branch from `main`:

   ```bash
   git checkout -b feat/my-feature
   ```

2. Make your changes and add or update tests as needed.
3. Ensure all CI checks pass locally per-commit (see Atomic
   commits above):

   ```bash
   just check
   ```

4. Push your branch and open a pull request against `main`.
5. Describe what the PR does, why it is needed, and how it was
   tested. Reference any related issues.

Pull requests require at least one review from a maintainer before
merging. The CI pipeline must be green on every commit.

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

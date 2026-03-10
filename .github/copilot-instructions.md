# Copilot Instructions for Dawon

These instructions apply to all Copilot interactions in this
repository: code review, chat, agent tasks, and PR generation.

---

## Project summary

Dawon is a Rust CLI tool that evaluates student C submissions from
the 42 School Piscine.  It is stricter than mini-moulinette:

- Norminette style check (subprocess)
- Symbol verification via `libloading`
- Forbidden-function scan (regex + `nm -u` symbol table)
- C compiler check: `cc -Wall -Wextra -Werror` (links, not
  `-fsyntax-only`)
- ASAN/UBSAN harness: fork+pipe binary protocol with SHA-256
  commitment comparison in Rust
- Valgrind: `--leak-check=full --error-exitcode=1` (Linux only)

---

## Configuration file

The config file is always `.dawon.toml`.  Never write `.gato.toml`.

---

## Harness design

The C harness uses `fork()` + `pipe()` + `waitpid()`.  It is
POSIX-only and cannot run on Windows.

- Rust code that calls `harness::run()` must be guarded with
  `#[cfg(unix)]` in tests.
- The capture buffer is 65 535 bytes.  Outputs larger than that
  are silently truncated.
- No expected bytes appear in the generated C source.  The Rust
  caller hashes captured stdout with SHA-256 and compares against
  the commitment stored in `TestCase.expected_sha256`.
- Protocol: per test — `[4 bytes big-endian length][N bytes stdout]`.

---

## SHA-256 commitment hashes

All expected outputs are stored as SHA-256 hashes, never as
plaintext strings.

To compute a commitment:

```
printf 'expected output' | sha256sum
```

Use the `hex!()` macro from `hex-literal`:

```rust
expected_sha256: &hex!("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"),
```

SHA-256 produces 32 bytes = **64 hexadecimal characters**.

---

## Platform guards

CI currently runs on Linux only.  The harness is POSIX-only and
will not compile on Windows.  macOS is welcome for local development
but is not in the CI matrix.

| Feature | Guard |
|---------|-------|
| Round-trip harness tests | `#[cfg(unix)]` |
| `detect_leaks=1` in ASAN_OPTIONS | Linux only — LSan not available on macOS |

If a multi-OS CI matrix is ever added:
- Gate Valgrind with `if: runner.os == 'Linux'`
- Gate norminette with `if: runner.os != 'Windows'`
- Do not set `ASAN_OPTIONS=detect_leaks=1` on macOS runners

---

## Commit discipline

- Follow **Conventional Commits 1.0.0**: `<type>[(<scope>)][!]: <description>`
- Subject line ≤ 72 characters
- Types: `feat` `fix` `docs` `style` `refactor` `perf` `test`
  `build` `ci` `chore` `revert` `init`
- Keep the subject line purely semantic; do not add lore to
  `<type>[(<scope>)][!]: <description>`
- Lore is optional flavour in the commit body after technical details
- Once the PR number is known, include it in commit metadata by
  appending `(#N)` to the subject line or adding `Closes #N` /
  `Refs #N` in the commit body footer
- Every commit merged to `main` must pass CI on its own (atomic
  commits — do not bundle a CI fix with unrelated changes)
- All commits must include `Signed-off-by:` (DCO); Monsieur
  Piscinette signature text is optional for external contributors

---

## Adding a subject

1. Create `src/subjects/<module>.rs` with `Subject` + `TestCase`
   definitions.
2. Include strict edge cases: `INT_MIN`, `INT_MAX`, null byte
   (`'\0'`), DEL (127).
3. Store all expected outputs as SHA-256 hashes (never plaintext).
4. Export in `src/subjects/mod.rs`.
5. Wire into `src/cli.rs` and `src/eval.rs`.
6. Update `CHANGELOG.md` under `[Unreleased]`.

---

## Adding a check

1. Create `src/checks/<name>.rs` returning `CheckResult`.
2. Export in `src/checks/mod.rs`.
3. Wire into `src/eval.rs`.
4. Write `#[cfg(test)]` unit tests in the module and
   `tests/test_<name>.rs` integration tests.
5. Update `CHANGELOG.md`.

---

## Code style

### Rust

- `cargo fmt` before every commit
- `cargo clippy -- -D warnings` — zero warnings allowed
- Doc comments use `///` for items, `//!` for modules
- Leave one blank line between top-level items

### General

- Prose (comments, docs, Markdown) wraps at 72 characters
- No unnecessary dependencies — prefer `std` where possible
- No bare `unwrap()` in production code (use `?` or explicit
  error handling)

---

## Pull request discipline

- Open an issue before writing code for non-trivial changes
- PR description must state: what, why, how tested
- Keep `Summary`, `Changes`, and `Test plan` factual; lore is optional
  in narrative sections
- CI must be green on every commit before merging
- Merge method: **Rebase and merge** (squash and merge are also
  allowed; merge commits are not)

---

## Feedback loop with @copilot

Every review comment, NACK, tACK, or concern posted in response
to a Copilot-opened PR or Copilot comment **must** address
`@copilot` directly.  This notifies the agent and keeps the
review thread active.

Examples:

```
@copilot NACK — remove the memcmp regression in check_output
before this can merge.

@copilot tACK 3f9a1c2 — tested locally, fmt + clippy + tests
all pass.

@copilot this approach leaks the pipe fd on error; close it
before returning Err.
```

Do not leave review concerns as unaddressed prose.  If the
concern is actionable, mention `@copilot` so the agent can
act on it.

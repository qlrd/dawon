<<<<<<< ours
# Dawon

[![CI](https://github.com/qlrd/dawon/actions/workflows/ci.yml/badge.svg)](https://github.com/qlrd/dawon/actions/workflows/ci.yml)
[![Rust 1.85+](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)

<p align="center">
  <img src="assets/dawon.png" alt="Dawon mascot" width="280"/>
</p>

> She does not move until the tests run.

---

Dawon is a super mini-moulinette for 42 school piscine exercises —
stricter than mini-moulinette, faster than waiting for the real thing.

Named after [Dawon](https://en.wikipedia.org/wiki/Dawon), the tiger
vahana of Mahadurga — guardian, merciless, thorough.

---

## Table of contents

- [What it does](#what-it-does)
- [Install](#install)
- [Usage](#usage)
- [Configuration](#configuration)
- [Output](#output)
- [Subjects](#subjects)
- [Development](#development)
- [Relationship to monsieur-ganesha](#relationship-to-monsieur-ganesha)
- [License](#license)

---

## What it does

Dawon runs a four-layer check pipeline on each exercise:

| Step | Check | Tool |
|------|-------|------|
| 1 | Symbol present | `libloading` (student `.so`) |
| 2 | Build | `cc -Wall -Wextra -Werror` with ASAN/UBSAN |
| 3 | Memory | `valgrind --leak-check=full` |
| 4 | Harness | fork + pipe, SHA-256 output comparison |

The harness tests edge cases that moulinette does not reveal
in advance. Test vectors are stored as SHA-256 commitments —
no expected outputs appear in this source.

Norminette and forbidden-function checks are handled by
[monsieur-ganesha](https://github.com/qlrd/monsieur-ganesha)
(the pre-commit hook suite) and are not repeated here.

---

## Install

```bash
cargo install --git https://github.com/qlrd/dawon
```

Or build from source:

```bash
git clone https://github.com/qlrd/dawon
cd dawon
cargo build --release
# binary at: target/release/dawon
```

---

## Usage

### Check your own submission

```bash
dawon check --path /path/to/C00
dawon check --path /path/to/C00 --exercise ex00
dawon check --path /path/to/Rush00 --rush
```

### Check a friend's submission

```bash
dawon friend --login aguiar    # looks in /home/aguiar/C00
dawon friend --login aguiar --module C01 --exercise ex01
dawon friend --path /path/to/friend/C00
```

### Flags

```
--no-sanitizers   skip ASAN/UBSAN (step 2 uses plain cc)
--no-valgrind     skip valgrind (step 3)
--rush            evaluate Rush00 subjects instead of C00
```

---

## Configuration

Create `.dawon.toml` in the module directory to override defaults:

```toml
[checks]
no_sanitizers = false
no_valgrind   = false
```

If `.dawon.toml` is absent, Dawon uses safe defaults (all checks
enabled).

---

## Output

```
════════════════════════════════════════════════════════════
  DAWON  super mini-moulinette
  Tiger of Mahadurga · Stricter than mini-moulinette
════════════════════════════════════════════════════════════

  Evaluating: myself  /home/student/C00

────────────────────────────────────────────────────────────
  ex00 — ft_putchar
  Write a function that outputs a char to stdout.
────────────────────────────────────────────────────────────
  [1/4] Symbol: ft_putchar                     PASS
  [2/4] Build (ASAN/UBSAN)                     PASS
  [3/4] Valgrind                               PASS
  [4/4] Harness                                PASS

  Summary: 4/4 passed

════════════════════════════════════════════════════════════
  GRAND SUMMARY
════════════════════════════════════════════════════════════
  ex00 (ft_putchar)   4/4
════════════════════════════════════════════════════════════
  4/4 checks passed
════════════════════════════════════════════════════════════
```

---

## Subjects

### C00

| Exercise | Function |
|----------|----------|
| ex00 | `ft_putchar` |
| ex01 | `ft_print_alphabet` |
| ex02 | `ft_print_reverse_alphabet` |
| ex03 | `ft_print_numbers` |
| ex04 | `ft_is_negative` |
| ex05 | `ft_print_comb` |
| ex06 | `ft_print_comb2` |
| ex07 | `ft_putnbr` |
| ex08 | `ft_print_combn` |

### Rush00

| Exercise | Files | Notes |
|----------|-------|-------|
| Rush00 | `main.c` | Static checks only (no harness vectors yet) |

---

## Development

```bash
cargo build
cargo test
cd tests/python && uv run pytest
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow.

---

## Relationship to monsieur-ganesha

[monsieur-ganesha](https://github.com/qlrd/monsieur-ganesha) is the
primary pre-commit hook suite for the piscine. Dawon is an optional
experimental companion: if mini-moulinette is unavailable or fails,
Dawon runs instead.

They are independent binaries. Dawon does not depend on
monsieur-ganesha and vice versa.

---

## License

[MIT](LICENSE)
=======
# dawon
Super mini-moulinette, stricter than moulinette
>>>>>>> theirs

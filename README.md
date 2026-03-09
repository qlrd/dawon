# Dawon

Dawon is a super mini-moulinette for 42 school piscine exercises —
stricter than mini-moulinette, faster than waiting for the real thing.

Named after [Dawon](https://en.wikipedia.org/wiki/Dawon), the tiger
vahana of Mahadurga — guardian, merciless, thorough.

---

## What it does

Dawon runs this check pipeline on each exercise:

| Step | Check | Tool |
|------|-------|------|
| 1 | Symbol present (optional) | `libloading` (student `.so`) |
| 2 | Memory | `valgrind --leak-check=full` |
| 3 | Harness | fork + pipe, SHA-256 with ASAN/UBSAN |

The harness tests edge cases moulinette does not: `INT_MIN`,
null bytes (`'\0'`), DEL (127), and all C(10,3) / C(100,2)
combinations for `ft_print_comb` / `ft_print_comb2`.

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
```

### Check a friend's submission

```bash
dawon friend --login aguiar    # looks in /home/aguiar/C00
dawon friend --login aguiar --module C01 --exercise ex01
dawon friend --path /path/to/friend/C00
```

### Flags

```
--no-sanitizers   build valgrind target without ASAN/UBSAN
--no-valgrind     skip valgrind
--check-symbol    enable symbol-presence check
```

---

## Configuration

Create `.dawon.toml` in the module directory to override defaults:

```toml
[checks]
no_sanitizers = false
no_valgrind   = false
check_symbol  = false
```

If `.dawon.toml` is absent, Dawon uses safe defaults.

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
  [1/2] Valgrind                               PASS
  [2/2] Function tests (7 passed)              PASS

  Summary: 2/2 passed

════════════════════════════════════════════════════════════
  GRAND SUMMARY
════════════════════════════════════════════════════════════
  ex00 (ft_putchar)   2/2
════════════════════════════════════════════════════════════
  2/2 checks passed
════════════════════════════════════════════════════════════
```

---

## Subjects

| Exercise | Function | Edge cases |
|----------|----------|------------|
| ex00 | `ft_putchar` | null byte, DEL (127) |
| ex01 | `ft_print_alphabet` | full a–z output |
| ex02 | `ft_print_reverse_alphabet` | full z–a output |
| ex03 | `ft_print_numbers` | full 0–9 output |
| ex04 | `ft_is_negative` | `INT_MIN`, 0, positive |
| ex05 | `ft_print_comb` | C(10,3) = 120 combinations |
| ex06 | `ft_print_comb2` | C(100,2) = 4950 pairs |
| ex07 | `ft_putnbr` | `INT_MIN`, 0, negative, max |
| ex08 | `ft_print_combn` | n=1 (digits), n=3 (=ft_print_comb) |

---

## Development

```bash
cargo build
cargo test
cd tests/python && uv run pytest
```

Fuzz (nightly required):

```bash
cargo +nightly fuzz run fuzz_forbidden
cargo +nightly fuzz run fuzz_config
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

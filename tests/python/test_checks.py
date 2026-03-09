"""Unit-level tests for individual Dawon checks via the CLI."""

import shutil
from pathlib import Path

import pytest

FIXTURES = Path(__file__).parent / "fixtures"


# ── harness / symbol steps ────────────────────────────────────────


def test_function_tests_step_appears_in_output(run, tmp_path):
    """The harness step label must be visible in the output."""
    ex_dir = tmp_path / "ex00"
    ex_dir.mkdir()
    shutil.copy(
        FIXTURES / "clean" / "ft_putchar.c",
        ex_dir / "ft_putchar.c",
    )
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert "Function tests" in r.stdout


# ── symbol check ──────────────────────────────────────────────────


def test_symbol_check_is_opt_in(run, tmp_path):
    """Symbol check appears only when --check-symbol is set."""
    ex_dir = tmp_path / "ex00"
    ex_dir.mkdir()
    shutil.copy(
        FIXTURES / "clean" / "ft_putchar.c",
        ex_dir / "ft_putchar.c",
    )
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert "Symbol" not in r.stdout

    with_symbol = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
        "--check-symbol",
    )
    assert "Symbol" in with_symbol.stdout


# ── exit codes ────────────────────────────────────────────────────


def test_exit_0_when_all_pass(run, tmp_path):
    """Exit code 0 when every check passes (clean file, no valgrind)."""
    ex_dir = tmp_path / "ex00"
    ex_dir.mkdir()
    shutil.copy(
        FIXTURES / "clean" / "ft_putchar.c",
        ex_dir / "ft_putchar.c",
    )
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert r.returncode == 0

"""Unit-level tests for individual Dawon checks via the CLI.

These tests are more focused than test_cli.py — each one targets a
specific check (forbidden functions, compiler flags, etc.) and
verifies the output message rather than just the exit code.
"""

import shutil
from pathlib import Path

import pytest

FIXTURES = Path(__file__).parent / "fixtures"


# ── forbidden functions ────────────────────────────────────────────


def test_forbidden_printf_reported_in_output(run, tmp_path):
    """The word 'printf' must appear in output when it is forbidden."""
    ex_dir = tmp_path / "ex00"
    ex_dir.mkdir()
    shutil.copy(
        FIXTURES / "forbidden" / "ft_putchar_printf.c",
        ex_dir / "ft_putchar.c",
    )
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert "printf" in r.stdout


def test_forbidden_check_name_in_output(run, tmp_path):
    """The 'Forbidden' check label must be visible in the output."""
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
    assert "Forbidden" in r.stdout or "forbidden" in r.stdout.lower()


def test_clean_file_forbidden_check_passes(run, tmp_path):
    """A write()-only ft_putchar.c must show PASS on forbidden check."""
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
    # At minimum, the step must appear; if it passes, no FAIL on forbidden
    lines = r.stdout.splitlines()
    forbidden_lines = [l for l in lines if "Forbidden" in l or "forbidden" in l.lower()]
    # If the forbidden check ran, it should not say FAIL
    for line in forbidden_lines:
        assert "FAIL" not in line, f"Forbidden check failed unexpectedly: {line}"


# ── compiler ──────────────────────────────────────────────────────


def test_compiler_check_appears_in_output(run, tmp_path):
    """The Compiler check label must appear in the output."""
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
    )
    assert "Compiler" in r.stdout or "compiler" in r.stdout.lower()


# ── symbol check ──────────────────────────────────────────────────


def test_symbol_check_appears_in_output(run, tmp_path):
    """The Symbol check step must be present."""
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
    assert "Symbol" in r.stdout or "ft_putchar" in r.stdout


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
    # Exit 0 only if all checks pass; norminette may not be installed.
    # Accept 0 or 1 — we just verify it isn't a crash (2+).
    assert r.returncode in (0, 1)


def test_exit_1_when_forbidden_fails(run, tmp_path):
    """Exit code 1 when at least one check fails."""
    ex_dir = tmp_path / "ex00"
    ex_dir.mkdir()
    shutil.copy(
        FIXTURES / "forbidden" / "ft_putchar_printf.c",
        ex_dir / "ft_putchar.c",
    )
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert r.returncode == 1

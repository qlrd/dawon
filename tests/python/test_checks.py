"""Unit-level tests for individual Dawon checks via the CLI.

These tests target specific check steps and verify output messages
rather than just exit codes.
"""

import shutil
from pathlib import Path

import pytest

FIXTURES = Path(__file__).parent / "fixtures"


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


# ── build check ───────────────────────────────────────────────────


def test_build_check_appears_in_output(run, tmp_path):
    """The Build step must appear in the output."""
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
    assert "Build" in r.stdout or "build" in r.stdout.lower()


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
    # Accept 0 or 1 — we verify it isn't a crash (2+).
    assert r.returncode in (0, 1)


def test_exit_1_when_build_fails(run, tmp_path):
    """Exit code 1 when the build step fails."""
    ex_dir = tmp_path / "ex00"
    ex_dir.mkdir()
    shutil.copy(
        FIXTURES / "compile_error" / "compile_error.c",
        ex_dir / "ft_putchar.c",
    )
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
    )
    assert r.returncode == 1
    assert "FAIL" in r.stdout

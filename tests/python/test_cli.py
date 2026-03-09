"""Functional tests for the dawon CLI binary.

These tests invoke the compiled binary as a subprocess and assert
on exit code and stdout/stderr content.  They cover:

- Version / help flags
- Missing directory handling
- clean submission → forbidden check passes
- printf-using submission → forbidden check fails
- --no-sanitizers flag accepted
- --no-valgrind flag accepted
"""

import shutil
from pathlib import Path

import pytest

FIXTURES = Path(__file__).parent / "fixtures"


# ── meta ──────────────────────────────────────────────────────────


def test_version(run):
    r = run("--version")
    assert r.returncode == 0
    assert "dawon" in r.stdout.lower() or "0." in r.stdout


def test_help_lists_subcommands(run):
    r = run("--help")
    assert r.returncode == 0
    assert "check" in r.stdout
    assert "friend" in r.stdout


def test_check_help(run):
    r = run("check", "--help")
    assert r.returncode == 0
    assert "--path" in r.stdout


def test_friend_help(run):
    r = run("friend", "--help")
    assert r.returncode == 0
    assert "--login" in r.stdout or "--path" in r.stdout


# ── check subcommand ───────────────────────────────────────────────


def test_check_empty_module_dir_missing_files(run, tmp_path):
    """Module dir exists but ex00/ is absent → missing-dir note."""
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert r.returncode == 1
    assert "ex00" in r.stdout.lower()


def test_check_clean_ft_putchar_forbidden_passes(run, tmp_path, module_dir):
    """A write()-only ft_putchar.c must pass the forbidden check."""
    module_dir("ex00", "ft_putchar.c", "clean/ft_putchar.c")
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert "ft_putchar" in r.stdout


def test_check_shows_exercise_name(run, tmp_path, module_dir):
    """dawon check output always mentions the exercise name."""
    module_dir("ex00", "ft_putchar.c", "clean/ft_putchar.c")
    r = run(
        "check",
        "--path", str(tmp_path),
        "--exercise", "ex00",
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert "ex00" in r.stdout


def test_check_printf_submission_fails(run, tmp_path):
    """ft_putchar using printf must fail the forbidden-functions check."""
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
    assert "FAIL" in r.stdout


def test_check_prints_pass_or_fail_for_each_step(run, tmp_path):
    """Every executed check step must print PASS, FAIL, SKIP, or ERROR."""
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
    combined = r.stdout + r.stderr
    assert any(tag in combined for tag in ("PASS", "FAIL", "SKIP", "ERROR"))


def test_no_sanitizers_flag_accepted(run, tmp_path):
    r = run("check", "--path", str(tmp_path), "--no-sanitizers")
    assert "unrecognized" not in r.stderr.lower()


def test_no_valgrind_flag_accepted(run, tmp_path):
    r = run("check", "--path", str(tmp_path), "--no-valgrind")
    assert "unrecognized" not in r.stderr.lower()


def test_rush_flag_listed_in_help(run):
    r = run("--help")
    assert r.returncode == 0
    assert "--rush" in r.stdout


def test_rush_flag_accepted(run, tmp_path):
    r = run("check", "--path", str(tmp_path), "--rush")
    assert "unrecognized" not in r.stderr.lower()


# ── grand summary ──────────────────────────────────────────────────


def test_output_contains_grand_summary(run, tmp_path):
    """dawon always prints a GRAND SUMMARY section."""
    r = run(
        "check",
        "--path", str(tmp_path),
        "--no-valgrind",
        "--no-sanitizers",
    )
    assert "GRAND SUMMARY" in r.stdout or "SUMMARY" in r.stdout


# ── banner ─────────────────────────────────────────────────────────


def test_banner_contains_dawon(run, tmp_path):
    r = run("check", "--path", str(tmp_path))
    assert "DAWON" in r.stdout

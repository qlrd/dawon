"""Pytest configuration and shared fixtures for Dawon CLI tests.

Requires Python >= 3.10.12.
The dawon binary must be compiled before running:
    cargo build
    pytest tests/python/
"""

import os
import shutil
import subprocess
from pathlib import Path

import pytest

# Resolve binary path: env var > debug build
_REPO_ROOT = Path(__file__).parent.parent.parent
DAWON_BIN = os.environ.get(
    "DAWON_BIN",
    str(_REPO_ROOT / "target" / "debug" / "dawon"),
)

FIXTURES = Path(__file__).parent / "fixtures"


@pytest.fixture(scope="session")
def bin_path() -> str:
    """Absolute path to the dawon binary."""
    if not Path(DAWON_BIN).exists():
        pytest.skip(
            f"dawon binary not found at {DAWON_BIN!r}. "
            "Run 'cargo build' first."
        )
    return DAWON_BIN


@pytest.fixture
def run(bin_path):
    """Return a helper that runs dawon with the given args.

    Usage::

        def test_version(run):
            r = run("--version")
            assert r.returncode == 0
    """

    def _run(*args, cwd=None, env=None, timeout=30):
        merged_env = {**os.environ, **(env or {})}
        return subprocess.run(
            [bin_path, *args],
            cwd=cwd,
            capture_output=True,
            text=True,
            timeout=timeout,
            env=merged_env,
        )

    return _run


@pytest.fixture
def module_dir(tmp_path):
    """Return a factory that creates a fake module directory.

    Usage::

        def test_clean(run, module_dir):
            d = module_dir("ex00", "ft_putchar.c", "clean/ft_putchar.c")
            r = run("check", "--path", str(d.parent), "--exercise", "ex00")
    """

    def _make(exercise: str, dest_name: str, fixture_relpath: str) -> Path:
        ex_dir = tmp_path / exercise
        ex_dir.mkdir(parents=True, exist_ok=True)
        src = FIXTURES / fixture_relpath
        shutil.copy(src, ex_dir / dest_name)
        return ex_dir

    return _make

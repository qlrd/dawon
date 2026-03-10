"""Tests for CI commit message validation helper."""

import importlib.util
from pathlib import Path

MODULE_PATH = (
    Path(__file__).parent.parent.parent
    / ".github"
    / "scripts"
    / "validate_commit_messages.py"
)
SPEC = importlib.util.spec_from_file_location(
    "validate_commit_messages", MODULE_PATH
)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC is not None and SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


def test_accepts_valid_conventional_subject_with_signoff():
    errors = MODULE.validate_commit(
        "abc123",
        "fix(ci): validate commit messages",
        "fix(ci): validate commit messages\n\nSigned-off-by: Jane <jane@x.io>",
    )
    assert errors == []


def test_rejects_non_conventional_subject():
    errors = MODULE.validate_commit(
        "abc123", "bad subject", "bad subject\n\nSigned-off-by: Jane <jane@x.io>"
    )
    assert any("does not match Conventional Commits" in error for error in errors)


def test_rejects_subject_over_72_chars():
    subject = "fix(ci): " + ("x" * 80)
    errors = MODULE.validate_commit(
        "abc123", subject, f"{subject}\n\nSigned-off-by: Jane <jane@x.io>"
    )
    assert any("max 72" in error for error in errors)


def test_rejects_missing_signed_off_by():
    errors = MODULE.validate_commit("abc123", "fix(ci): keep subject valid", "body")
    assert any("missing Signed-off-by" in error for error in errors)

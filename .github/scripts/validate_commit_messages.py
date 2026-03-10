"""Validate commit messages against repository contribution rules."""

from __future__ import annotations

import os
import re
import subprocess
import sys

CONVENTIONAL_SUBJECT = re.compile(
    r"^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert|init|lore)(\([^)]+\))?(!)?: \S.*"
)
SIGNED_OFF_BY = re.compile(r"^Signed-off-by:\s+\S.*", re.MULTILINE)
MAX_SUBJECT_LEN = 72


def _git(*args: str) -> str:
    result = subprocess.run(
        ["git", *args], check=True, capture_output=True, text=True
    )
    return result.stdout.strip()


def _is_zero_sha(value: str) -> bool:
    return bool(value) and set(value) == {"0"}


def _commit_shas(base_sha: str, head_sha: str) -> list[str]:
    if _is_zero_sha(base_sha):
        return [head_sha]

    commits = _git("rev-list", "--reverse", f"{base_sha}..{head_sha}")
    if not commits:
        return [head_sha]
    return commits.splitlines()


def validate_commit(sha: str, subject: str, body: str) -> list[str]:
    errors: list[str] = []
    if not CONVENTIONAL_SUBJECT.match(subject):
        errors.append(
            f"{sha}: subject does not match Conventional Commits: {subject!r}"
        )
    if len(subject) > MAX_SUBJECT_LEN:
        errors.append(
            f"{sha}: subject is {len(subject)} chars (max {MAX_SUBJECT_LEN})"
        )
    if not SIGNED_OFF_BY.search(body):
        errors.append(f"{sha}: missing Signed-off-by trailer")
    return errors


def main() -> int:
    base_sha = os.environ["BASE_SHA"]
    head_sha = os.environ["HEAD_SHA"]
    failures: list[str] = []

    for sha in _commit_shas(base_sha, head_sha):
        subject = _git("show", "-s", "--format=%s", sha)
        body = _git("show", "-s", "--format=%B", sha)
        failures.extend(validate_commit(sha, subject, body))

    if failures:
        print("Commit message validation failed:")
        for failure in failures:
            print(f"- {failure}")
        return 1

    print("All commit messages passed validation.")
    return 0


if __name__ == "__main__":
    sys.exit(main())

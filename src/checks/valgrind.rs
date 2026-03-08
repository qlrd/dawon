//! Valgrind memory check — stricter than moulinette.
//!
//! Runs `valgrind --leak-check=full --show-leak-kinds=all
//! --error-exitcode=1` on the compiled binary.
//! Moulinette does not always run valgrind; we always do.

use std::path::Path;
use std::process::Command;
use std::time::Duration;

use crate::report::CheckResult;

/// Run valgrind on *binary*.
///
/// Returns `CheckResult::skip` if valgrind is not installed.
pub fn check(binary: &Path, timeout: Duration) -> CheckResult {
    let _ = timeout; // enforced externally via SIGKILL if needed

    let out = match Command::new("valgrind")
        .args([
            "--leak-check=full",
            "--show-leak-kinds=all",
            "--track-origins=yes",
            "--error-exitcode=1",
            "--quiet",
        ])
        .arg(binary)
        .output()
    {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return CheckResult::skip("Valgrind", "valgrind not installed");
        }
        Err(e) => return CheckResult::error("Valgrind", e.to_string()),
        Ok(o) => o,
    };

    if out.status.success() {
        CheckResult::pass("Valgrind")
    } else {
        let msgs: Vec<String> = String::from_utf8_lossy(&out.stderr)
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(String::from)
            .collect();
        CheckResult::fail("Valgrind", msgs)
    }
}

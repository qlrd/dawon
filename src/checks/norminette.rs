//! Norminette check — subprocess call.

use std::path::Path;
use std::process::Command;

use crate::report::CheckResult;

pub fn check(files: &[&Path]) -> CheckResult {
    if files.is_empty() {
        return CheckResult::pass("Norminette");
    }

    let out = match Command::new("norminette").args(files).output() {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return CheckResult::skip("Norminette", "norminette not installed");
        }
        Err(e) => return CheckResult::error("Norminette", e.to_string()),
        Ok(o) => o,
    };

    if out.status.success() {
        CheckResult::pass("Norminette")
    } else {
        let stderr = String::from_utf8_lossy(&out.stdout).to_string();
        let msgs = stderr
            .lines()
            .filter(|l| l.contains("Error"))
            .map(String::from)
            .collect();
        CheckResult::fail("Norminette", msgs)
    }
}

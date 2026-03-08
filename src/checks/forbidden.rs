//! Forbidden-function detection with two layers.
//!
//! Layer 1 — regex scan: fast, catches obvious calls.
//! Layer 2 — nm symbol table: catches calls through macros or
//!            unusual whitespace that regex might miss.
//!
//! Both layers are stricter than moulinette's single regex pass.

use std::path::Path;
use std::process::Command;

use regex::Regex;

use crate::report::CheckResult;

/// Run both the regex scan and nm analysis on *source*.
pub fn check(source: &Path, object: Option<&Path>, forbidden: &[&str]) -> CheckResult {
    if forbidden.is_empty() {
        return CheckResult::pass("Forbidden functions");
    }

    let mut msgs: Vec<String> = Vec::new();

    // ── Layer 1: regex scan ──────────────────────────────────────
    if let Err(e) = regex_scan(source, forbidden, &mut msgs) {
        return CheckResult::error("Forbidden functions", e.to_string());
    }

    // ── Layer 2: nm symbol table ─────────────────────────────────
    if let Some(obj) = object {
        nm_scan(obj, forbidden, &mut msgs);
    }

    if msgs.is_empty() {
        CheckResult::pass("Forbidden functions")
    } else {
        CheckResult::fail("Forbidden functions", msgs)
    }
}

fn regex_scan(source: &Path, forbidden: &[&str], out: &mut Vec<String>) -> anyhow::Result<()> {
    let text = std::fs::read_to_string(source)?;
    let alternation: Vec<String> = forbidden.iter().map(|f| regex::escape(f)).collect();
    let pat = format!(r"\b({})\s*\(", alternation.join("|"));
    let re = Regex::new(&pat)?;

    for (lineno, line) in text.lines().enumerate() {
        // strip inline comments before scanning
        let code = line.split("//").next().unwrap_or(line);
        for m in re.captures_iter(code) {
            out.push(format!(
                "{}:{}: [regex] função proibida '{}'",
                source.display(),
                lineno + 1,
                &m[1]
            ));
        }
    }
    Ok(())
}

/// Run `nm -u <object>` and flag any undefined symbol that matches
/// a forbidden function name exactly.
fn nm_scan(object: &Path, forbidden: &[&str], out: &mut Vec<String>) {
    let result = Command::new("nm").arg("-u").arg(object).output();
    let Ok(output) = result else { return };
    if !output.status.success() {
        return;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for sym in stdout.lines() {
        // nm output: "                 U symbol_name"
        let name = sym.trim().trim_start_matches('U').trim();
        // Strip leading underscore (macOS / some linkers)
        let name = name.strip_prefix('_').unwrap_or(name);
        for &f in forbidden {
            if name == f {
                out.push(format!(
                    "[nm] '{}' linked as undefined symbol — forbidden",
                    f
                ));
            }
        }
    }
}

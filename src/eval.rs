//! Evaluation runner — runs the full check suite for one exercise.

use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::checks::{compiler, harness, valgrind};
use crate::config::Config;
use crate::report::{CheckResult, SuiteResult};
use crate::subjects::Subject;

/// Resolve every required source file inside *exercise_dir*.
/// Returns `None` (with the list of missing files) if any are absent.
pub fn locate_files(subject: &Subject, exercise_dir: &Path) -> Result<Vec<PathBuf>, Vec<String>> {
    let mut paths = Vec::new();
    let mut missing = Vec::new();
    for &file in subject.files {
        let p = exercise_dir.join(file);
        if p.exists() {
            paths.push(p);
        } else {
            missing.push(format!("missing: {file}"));
        }
    }
    if missing.is_empty() {
        Ok(paths)
    } else {
        Err(missing)
    }
}

/// Find a friend's module directory from their login.
///
/// Searches the most common 42-school layouts in order:
///   /home/<login>/<module>/
///   /home/<login>/<module_lower>/
pub fn find_friend_path(login: &str, module: &str) -> Option<PathBuf> {
    let candidates = [
        format!("/home/{login}/{module}"),
        format!("/home/{login}/{}", module.to_lowercase()),
    ];
    candidates
        .into_iter()
        .map(PathBuf::from)
        .find(|p| p.is_dir())
}

/// Run the full check suite for *subject* against *exercise_dir*.
pub fn run(subject: &Subject, exercise_dir: &Path, cfg: &Config) -> SuiteResult {
    let mut checks: Vec<CheckResult> = Vec::new();

    // ── locate source files ──────────────────────────────────────
    let source_files = match locate_files(subject, exercise_dir) {
        Ok(f) => f,
        Err(missing) => {
            checks.push(CheckResult::fail("Files present", missing));
            return SuiteResult {
                exercise: subject.exercise.to_string(),
                function: subject.function.to_string(),
                checks,
            };
        }
    };
    let source_paths: Vec<&Path> = source_files.iter().map(PathBuf::as_path).collect();

    // ── 1. optional symbol name (libloading) ─────────────────────
    if cfg.checks.check_symbol {
        checks.push(harness::check_symbol(source_paths[0], subject.function));
    }

    // ── 2. valgrind ───────────────────────────────────────────────
    if !cfg.checks.no_valgrind {
        let tmp = match tempfile::TempDir::new() {
            Ok(t) => t,
            Err(e) => {
                checks.push(CheckResult::error("Setup", e.to_string()));
                return SuiteResult {
                    exercise: subject.exercise.to_string(),
                    function: subject.function.to_string(),
                    checks,
                };
            }
        };

        let sanitize = !cfg.checks.no_sanitizers;
        let binary = tmp.path().join("student_bin");
        let compile_res = compiler::compile(&source_paths, &binary, sanitize);
        if compile_res.is_pass() {
            checks.push(valgrind::check(&binary, Duration::from_secs(10)));
        } else {
            checks.push(CheckResult {
                name: "Valgrind (build)".to_string(),
                status: compile_res.status,
            });
        }
    }

    // ── 3. function harness ───────────────────────────────────────
    match harness::run(subject, &source_files) {
        Ok(r) => checks.push(r),
        Err(e) => checks.push(CheckResult::error("Function tests", e.to_string())),
    }

    SuiteResult {
        exercise: subject.exercise.to_string(),
        function: subject.function.to_string(),
        checks,
    }
}

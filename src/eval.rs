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

    // ── 1. symbol name (libloading) ──────────────────────────────
    if subject.tests.is_empty() {
        checks.push(CheckResult::skip(
            "Symbol check",
            "not required for this subject",
        ));
    } else {
        checks.push(harness::check_symbol(source_paths[0], subject.function));
    }

    // ── 2. build with ASAN/UBSAN (prerequisite for harness + valgrind) ──
    let tmp = match tempfile::TempDir::new() {
        Ok(t) => t,
        Err(e) => {
            checks.push(CheckResult::error("Build", e.to_string()));
            return SuiteResult {
                exercise: subject.exercise.to_string(),
                function: subject.function.to_string(),
                checks,
            };
        }
    };

    let sanitize = !cfg.checks.no_sanitizers;
    let binary = tmp.path().join("student_bin");
    let build_res = compiler::compile(&source_paths, &binary, sanitize);
    let build_ok = build_res.is_pass();
    checks.push(build_res);

    // ── 3. valgrind ──────────────────────────────────────────────
    if build_ok && !cfg.checks.no_valgrind {
        checks.push(valgrind::check(&binary, Duration::from_secs(10)));
    }

    // ── 4. function harness ──────────────────────────────────────
    if build_ok {
        if subject.tests.is_empty() {
            checks.push(CheckResult::skip(
                "Function tests",
                "no harness vectors defined for this subject",
            ));
        } else {
            match harness::run(subject, &source_files) {
                Ok(r) => checks.push(r),
                Err(e) => checks.push(CheckResult::error("Function tests", e.to_string())),
            }
        }
    }

    SuiteResult {
        exercise: subject.exercise.to_string(),
        function: subject.function.to_string(),
        checks,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::run;
    use crate::config::Config;
    use crate::report::CheckStatus;
    use crate::subjects;

    #[test]
    fn test_skips_symbol_and_harness_when_no_test_vectors() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let ex_dir = tmp.path().join("Rush00");
        fs::create_dir_all(&ex_dir).expect("create rush dir");
        fs::write(ex_dir.join("main.c"), "int main(void) { return (0); }\n").expect("write C");

        let mut cfg = Config::default();
        cfg.checks.no_sanitizers = true;
        cfg.checks.no_valgrind = true;

        let result = run(&subjects::rush00::ALL[0], &ex_dir, &cfg);

        let symbol = result
            .checks
            .iter()
            .find(|c| c.name == "Symbol check")
            .expect("symbol check present");
        assert!(matches!(symbol.status, CheckStatus::Skipped(_)));

        let harness = result
            .checks
            .iter()
            .find(|c| c.name == "Function tests")
            .expect("function tests check present");
        assert!(matches!(harness.status, CheckStatus::Skipped(_)));
    }
}

//! Compiler check — cc with strict flags and optional sanitizers.
//!
//! Stricter than moulinette: adds -fsanitize=address,undefined so
//! the harness binary runs under ASAN/UBSAN automatically.

use std::path::Path;
use std::process::Command;

use crate::report::CheckResult;

/// Compile *source_files* to *output*.
///
/// When `sanitizers` is true, adds `-fsanitize=address,undefined -g`
/// so any binary linked from this object inherits the sanitizers.
pub fn compile(source_files: &[&Path], output: &Path, sanitizers: bool) -> CheckResult {
    let mut cmd = Command::new("cc");
    cmd.args(["-Wall", "-Wextra", "-Werror"]);
    if sanitizers {
        cmd.args(["-fsanitize=address,undefined", "-g"]);
    }
    cmd.args(source_files).arg("-o").arg(output);

    let out = match cmd.output() {
        Err(e) => return CheckResult::error("Compiler", e.to_string()),
        Ok(o) => o,
    };

    if out.status.success() {
        CheckResult::pass("Compiler")
    } else {
        let msgs: Vec<String> = String::from_utf8_lossy(&out.stderr)
            .lines()
            .map(String::from)
            .collect();
        CheckResult::fail("Compiler", msgs)
    }
}

/// Compile to object file (for symbol table analysis via nm).
pub fn compile_to_object(source: &Path, output: &Path) -> CheckResult {
    let out = match Command::new("cc")
        .args(["-Wall", "-Wextra", "-Werror", "-c"])
        .arg(source)
        .arg("-o")
        .arg(output)
        .output()
    {
        Err(e) => return CheckResult::error("Compiler (object)", e.to_string()),
        Ok(o) => o,
    };

    if out.status.success() {
        CheckResult::pass("Compiler (object)")
    } else {
        let msgs: Vec<String> = String::from_utf8_lossy(&out.stderr)
            .lines()
            .map(String::from)
            .collect();
        CheckResult::fail("Compiler (object)", msgs)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{compile, compile_to_object};
    use crate::report::CheckStatus;

    #[test]
    fn compile_passes_for_valid_source() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let src = tmp.path().join("ok.c");
        let bin = tmp.path().join("ok_bin");
        fs::write(&src, "int main(void) { return (0); }\n").expect("write C");

        let result = compile(&[src.as_path()], &bin, false);

        assert!(matches!(result.status, CheckStatus::Pass));
        assert!(bin.exists());
    }

    #[test]
    fn compile_fails_for_invalid_source() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let src = tmp.path().join("bad.c");
        let bin = tmp.path().join("bad_bin");
        fs::write(&src, "int main(void) { return ; }\n").expect("write C");

        let result = compile(&[src.as_path()], &bin, false);

        assert!(matches!(result.status, CheckStatus::Fail(_)));
    }

    #[test]
    fn compile_to_object_passes_for_valid_source() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let src = tmp.path().join("ok_obj.c");
        let obj = tmp.path().join("ok_obj.o");
        fs::write(&src, "int value(void) { return (42); }\n").expect("write C");

        let result = compile_to_object(&src, &obj);

        assert!(matches!(result.status, CheckStatus::Pass));
        assert!(obj.exists());
    }
}

//! C test harness generator + runner.
//!
//! For each exercise, Dawon generates a C file that:
//! 1. Declares the student function prototype.
//! 2. Wraps each test case in `static void test_N(void)`.
//! 3. Uses `dawon_capture(test_N)` to fork, capture stdout via a
//!    pipe, and emit a 4-byte big-endian length followed by the raw
//!    bytes to the parent's stdout — one record per test.
//! 4. Compiles student code + harness with ASAN/UBSAN.
//! 5. Rust reads the binary protocol, SHA-256 hashes each captured
//!    output, and compares against the stored commitment.
//!
//! No expected bytes appear in the generated C source.
//!
//! Protocol: for each test the harness writes to stdout —
//!   `[4 bytes big-endian length][length bytes of captured stdout]`
//!
//! Capture limit: `dawon_capture` drains up to 65 535 bytes from the
//! child's stdout in a loop.  If the child writes more, the read-end
//! is closed (triggering SIGPIPE) before `waitpid`, preventing a
//! deadlock; the child is reaped normally and only the captured
//! portion is used.  All C00–C09 exercises produce far less than that.
//!
//! Strictness beyond mini-moulinette:
//! - Test isolation via fork() prevents state pollution.
//! - ASAN catches heap overflows / use-after-free.
//! - UBSAN catches signed integer overflow (INT_MIN trap!).
//! - SHA-256 comparison handles embedded null bytes correctly.

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::report::CheckResult;
use crate::subjects::{Subject, TestCase};

// ── C infrastructure injected into every harness ──────────────────
//
// Protocol: for each test the harness writes to stdout —
//   [4 bytes big-endian length][length bytes of captured stdout]
// No expected bytes are embedded anywhere in the generated C.
// The Rust caller computes SHA-256 of each captured output and
// compares it against the commitment stored in TestCase.

static HARNESS_INFRA: &str = r#"
static void dawon_capture(void (*fn)(void))
{
    int             pipefd[2];
    pid_t           pid;
    int             wstatus;
    unsigned char   buf[65536];
    ssize_t         n = 0;
    ssize_t         r;
    unsigned char   hdr[4];

    if (pipe(pipefd) == 0)
    {
        pid = fork();
        if (pid == 0)
        {
            close(pipefd[0]);
            dup2(pipefd[1], 1);
            close(pipefd[1]);
            fn();
            _exit(0);
        }
        else if (pid > 0)
        {
            close(pipefd[1]);
            while (n < (ssize_t)(sizeof(buf) - 1))
            {
                r = read(pipefd[0], buf + n,
                         sizeof(buf) - 1 - (size_t)n);
                if (r <= 0)
                    break;
                n += r;
            }
            close(pipefd[0]);
            waitpid(pid, &wstatus, 0);
        }
        else
        {
            close(pipefd[0]);
            close(pipefd[1]);
        }
    }
    hdr[0] = (unsigned char)((n >> 24) & 0xFF);
    hdr[1] = (unsigned char)((n >> 16) & 0xFF);
    hdr[2] = (unsigned char)((n >>  8) & 0xFF);
    hdr[3] = (unsigned char)( n        & 0xFF);
    write(1, hdr, 4);
    if (n > 0)
        write(1, buf, (size_t)n);
}
"#;

// ── harness code generation ────────────────────────────────────────

/// Generate the full C harness source for *subject* / *tests*.
///
/// No expected bytes are embedded in the generated C — the harness
/// only captures stdout and sends it back via a length-prefixed
/// binary protocol. SHA-256 comparison happens entirely in Rust.
pub fn generate(subject: &Subject, tests: &[TestCase]) -> String {
    let mut src = String::new();

    src.push_str("#include <unistd.h>\n");
    src.push_str("#include <stdlib.h>\n");
    src.push_str("#include <stdio.h>\n");
    src.push_str("#include <string.h>\n");
    src.push_str("#include <sys/wait.h>\n");
    src.push_str("#include <limits.h>\n\n");

    // Student declaration
    src.push_str(subject.c_prototype);
    src.push_str("\n\n");

    // Infrastructure (capture only — no expected values)
    src.push_str(HARNESS_INFRA);

    // Per-test wrapper functions
    for (i, tc) in tests.iter().enumerate() {
        src.push_str(&format!(
            "static void test_{i}(void) {{ {call} }}\n",
            call = tc.c_call
        ));
    }
    src.push('\n');

    // main: capture each test's stdout via binary protocol
    src.push_str("int\tmain(void)\n{\n");
    for i in 0..tests.len() {
        src.push_str(&format!("\tdawon_capture(test_{i});\n"));
    }
    src.push_str("\treturn (0);\n}\n");

    src
}

// ── compilation + execution ────────────────────────────────────────

/// Run the harness for *subject* against the student *source_files*.
///
/// Returns a `CheckResult` describing which test cases passed/failed
/// and whether any ASAN/UBSAN violations were detected.
pub fn run(subject: &Subject, source_files: &[PathBuf]) -> anyhow::Result<CheckResult> {
    let tmp = tempfile::TempDir::new()?;

    let harness_path = tmp.path().join("dawon_harness.c");
    std::fs::write(&harness_path, generate(subject, subject.tests))?;

    let binary = tmp.path().join("dawon_test");
    let compile_out = Command::new("cc")
        .args([
            "-Wall",
            "-Wextra",
            "-Werror",
            "-fsanitize=address,undefined",
            "-g",
        ])
        .args(source_files)
        .arg(&harness_path)
        .arg("-o")
        .arg(&binary)
        .output()?;

    if !compile_out.status.success() {
        let msgs: Vec<String> = String::from_utf8_lossy(&compile_out.stderr)
            .lines()
            .map(String::from)
            .collect();
        return Ok(CheckResult::fail("Function tests (harness)", msgs));
    }

    // Run — binary protocol on stdout, ASAN/UBSAN errors on stderr.
    // detect_leaks=1 is Linux-only (LSan not available on macOS); valgrind
    // handles leak detection on Linux separately, so omit it here.
    let run_out = Command::new(&binary)
        .env("ASAN_OPTIONS", "log_path=/dev/stderr")
        .env("UBSAN_OPTIONS", "print_stacktrace=1")
        .output()?;

    compare_outputs(&run_out.stdout, &run_out.stderr, subject.tests)
}

/// Compare captured test outputs against SHA-256 commitments.
///
/// Reads the length-prefixed binary protocol from `stdout`, hashes
/// each captured output with SHA-256, and compares against the
/// commitment stored in each `TestCase`. Expected bytes never appear
/// in this function — only the hash comparison result.
fn compare_outputs(
    stdout: &[u8],
    stderr: &[u8],
    tests: &[TestCase],
) -> anyhow::Result<CheckResult> {
    // Detect ASAN/UBSAN errors in stderr first
    let stderr_text = String::from_utf8_lossy(stderr);
    let asan_errors: Vec<String> = stderr_text
        .lines()
        .filter(|l| l.contains("ERROR") || l.contains("runtime error"))
        .map(String::from)
        .collect();

    let mut pos = 0usize;
    let mut msgs: Vec<String> = Vec::new();
    let mut pass_count = 0usize;

    for tc in tests {
        if pos + 4 > stdout.len() {
            // Once the stream is out of sync, further parsing is meaningless.
            msgs.push(format!("  ERROR  {} — truncated protocol", tc.name));
            break;
        }
        let len = u32::from_be_bytes([
            stdout[pos],
            stdout[pos + 1],
            stdout[pos + 2],
            stdout[pos + 3],
        ]) as usize;
        pos += 4;

        if pos + len > stdout.len() {
            msgs.push(format!("  ERROR  {} — truncated body", tc.name));
            break;
        }
        let output = &stdout[pos..pos + len];
        pos += len;

        let hash = Sha256::digest(output);
        if hash.as_slice() == tc.expected_sha256.as_slice() {
            pass_count += 1;
            msgs.push(format!("  PASS  {}", tc.name));
        } else {
            msgs.push(format!("  FAIL  {}", tc.name));
        }
    }

    if !asan_errors.is_empty() {
        msgs.extend(asan_errors);
        return Ok(CheckResult::fail("Function tests + ASAN/UBSAN", msgs));
    }

    let all_pass = msgs.iter().all(|l| l.trim_start().starts_with("PASS"));
    if all_pass && pass_count > 0 {
        Ok(CheckResult::pass(format!(
            "Function tests ({pass_count} passed)"
        )))
    } else {
        Ok(CheckResult::fail("Function tests", msgs))
    }
}

// ── symbol existence check via libloading ─────────────────────────

/// Compile *source* as a shared library and verify that the expected
/// function symbol is exported with the correct name.
///
/// This catches the common mistake of misspelling the function name
/// (e.g. `ft_putChar` instead of `ft_putchar`).
pub fn check_symbol(source: &Path, function_name: &str) -> CheckResult {
    let tmp = match tempfile::TempDir::new() {
        Ok(t) => t,
        Err(e) => return CheckResult::error("Symbol check", e.to_string()),
    };

    let lib_path = tmp.path().join("libstudent.so");
    let out = match Command::new("cc")
        .args(["-shared", "-fPIC", "-o"])
        .arg(&lib_path)
        .arg(source)
        .output()
    {
        Err(e) => return CheckResult::error("Symbol check", e.to_string()),
        Ok(o) => o,
    };

    if !out.status.success() {
        return CheckResult::error(
            "Symbol check",
            "failed to build shared library for symbol verification",
        );
    }

    let result = unsafe {
        libloading::Library::new(&lib_path).and_then(|lib| {
            let _sym: libloading::Symbol<*const ()> = lib.get(function_name.as_bytes())?;
            Ok(())
        })
    };

    match result {
        Ok(()) => CheckResult::pass(format!("Symbol '{function_name}' exported")),
        Err(e) => CheckResult::fail(
            "Symbol check",
            vec![format!("'{function_name}' not found: {e}")],
        ),
    }
}

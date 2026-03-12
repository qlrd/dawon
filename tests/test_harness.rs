//! Rust integration tests for the C harness generator.
//!
//! These tests verify that `harness::generate()` produces valid C
//! source and that the generated harness compiles and runs correctly.
//! A synthetic function (`ft_gato_probe`) is used — it is NOT a
//! piscine exercise, so no exercise answer is revealed here.
//!
//! SHA-256 commitments used here can be verified externally:
//!   printf 'a' | sha256sum   →  ca978112...
//!   printf 'b' | sha256sum   →  3e23e816...

use dawon::checks::harness;
use dawon::report::CheckStatus;
use dawon::subjects::{Subject, TestCase};
use hex_literal::hex;

// ── synthetic subject used only for harness infrastructure tests ────

static PROBE_TESTS: &[TestCase] = &[
    TestCase {
        name: "'a'",
        c_call: "ft_gato_probe('a');",
        expected_sha256: &hex!("ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"),
    },
    TestCase {
        name: "'b'",
        c_call: "ft_gato_probe('b');",
        expected_sha256: &hex!("3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d"),
    },
];

static PROBE_SUBJECT: Subject = Subject {
    exercise: "test_only",
    function: "ft_gato_probe",
    c_prototype: "void\tft_gato_probe(char c);",
    files: &["ft_gato_probe.c"],
    description: "harness infrastructure probe — not a piscine exercise",
    tests: PROBE_TESTS,
};

// ── structural tests (no compilation) ──────────────────────────────

#[test]
fn generate_contains_prototype() {
    let src = harness::generate(&PROBE_SUBJECT, PROBE_TESTS);
    assert!(src.contains(PROBE_SUBJECT.c_prototype));
}

#[test]
fn generate_contains_all_test_wrappers() {
    let src = harness::generate(&PROBE_SUBJECT, PROBE_TESTS);
    for i in 0..PROBE_TESTS.len() {
        assert!(src.contains(&format!("test_{i}(")));
    }
}

#[test]
fn generate_contains_main() {
    let src = harness::generate(&PROBE_SUBJECT, PROBE_TESTS);
    assert!(src.contains("int\tmain(void)"));
    assert!(src.contains("dawon_capture(test_0)"));
}

#[test]
fn generate_contains_no_expected_bytes() {
    // The generated C must never contain expected answer bytes.
    // dawon_capture takes no expected argument — only a function pointer.
    let src = harness::generate(&PROBE_SUBJECT, PROBE_TESTS);
    assert!(!src.contains("exp_0"));
    assert!(src.contains("dawon_capture(test_0)"));
}

// ── round-trip tests (compilation, Unix only) ──────────────────────
//
// The generated harness uses fork()/pipe()/waitpid() and
// #include <sys/wait.h> — POSIX-only constructs unavailable on
// Windows.  The structural tests above are platform-independent.

#[cfg(unix)]
fn write_probe_impl(dir: &std::path::Path) -> std::path::PathBuf {
    let src = dir.join("ft_gato_probe.c");
    std::fs::write(
        &src,
        b"#include <unistd.h>\nvoid ft_gato_probe(char c) { write(1, &c, 1); }\n",
    )
    .unwrap();
    src
}

#[cfg(unix)]
fn write_wrong_probe_impl(dir: &std::path::Path) -> std::path::PathBuf {
    let src = dir.join("ft_gato_probe.c");
    std::fs::write(
        &src,
        b"#include <unistd.h>\nvoid ft_gato_probe(char c) { (void)c; write(1, \"?\", 1); }\n",
    )
    .unwrap();
    src
}

#[cfg(unix)]
#[test]
fn harness_passes_when_probe_output_matches() {
    let tmp = tempfile::TempDir::new().unwrap();
    let src = write_probe_impl(tmp.path());
    let result = harness::run(&PROBE_SUBJECT, &[src]).unwrap();
    assert!(result.is_pass(), "probe should pass: {:?}", result.status);
}

#[cfg(unix)]
#[test]
fn harness_fails_when_probe_output_is_wrong() {
    let tmp = tempfile::TempDir::new().unwrap();
    let src = write_wrong_probe_impl(tmp.path());
    let result = harness::run(&PROBE_SUBJECT, &[src]).unwrap();
    assert!(!result.is_pass(), "probe should fail with wrong output");
    let CheckStatus::Fail(msgs) = &result.status else {
        panic!("expected failure messages, got {:?}", result.status);
    };
    assert!(msgs.iter().any(|m| m == "  FAIL  'a'"));
    assert!(
        msgs.iter()
            .any(|m| m.contains("'a': actual: \"?\" (1 byte)")),
        "missing actual output detail for failing test: {:?}",
        msgs
    );
    assert!(
        !msgs.iter().any(|m| m.contains("ca978112")),
        "failure output must not reveal expected hash: {:?}",
        msgs
    );
}

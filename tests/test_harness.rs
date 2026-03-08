//! Rust integration tests for the C harness generator.
//!
//! These tests verify that `harness::generate()` produces valid C
//! source and that the generated harness compiles and runs correctly.
//! A synthetic function (`ft_gato_probe`) is used — it is NOT a
//! piscine exercise, so no exercise answer is revealed here.

use dawon::checks::harness;
use hex_literal::hex;

use dawon::subjects::{Subject, TestCase};

// ── synthetic subject used only for harness infrastructure tests ────
//
// ft_gato_probe is NOT a piscine exercise.  The commitments below are
// SHA-256 of single ASCII bytes and can be verified with:
//   printf 'a' | sha256sum   →  ca978112...
//   printf 'z' | sha256sum   →  594e519a...

static PROBE_TESTS: &[TestCase] = &[
    TestCase {
        name: "'a'",
        c_call: "ft_gato_probe('a');",
        expected_sha256: &hex!("ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"),
    },
    TestCase {
        name: "'z'",
        c_call: "ft_gato_probe('z');",
        expected_sha256: &hex!("594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06"),
    },
];

static PROBE_SUBJECT: Subject = Subject {
    exercise: "test_only",
    function: "ft_gato_probe",
    c_prototype: "void\tft_gato_probe(char c);",
    files: &["ft_gato_probe.c"],
    forbidden: &[],
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
}

#[test]
fn generate_contains_no_expected_bytes() {
    // The generated C must never contain expected answer bytes.
    // gato_capture takes no expected argument — only a function pointer.
    let src = harness::generate(&PROBE_SUBJECT, PROBE_TESTS);
    assert!(!src.contains("exp_0"));
    assert!(src.contains("gato_capture(test_0)"));
}

// ── round-trip tests (compilation) ─────────────────────────────────

#[test]
fn harness_passes_when_probe_output_matches() {
    use std::io::Write as _;

    // Correct probe: write the char to stdout.
    // This is NOT a piscine exercise answer.
    let mut src_file = tempfile::Builder::new()
        .suffix(".c")
        .tempfile()
        .expect("tempfile");
    src_file
        .write_all(b"#include <unistd.h>\nvoid\tft_gato_probe(char c) { write(1, &c, 1); }\n")
        .expect("write");

    let result = harness::run(&PROBE_SUBJECT, &[src_file.path().to_path_buf()])
        .expect("harness::run should not error");

    assert!(result.is_pass(), "probe should pass: {:?}", result.status);
}

#[test]
fn harness_fails_when_probe_output_is_wrong() {
    use std::io::Write as _;

    // Incorrect probe: always writes the wrong byte.
    let mut src_file = tempfile::Builder::new()
        .suffix(".c")
        .tempfile()
        .expect("tempfile");
    src_file
        .write_all(
            b"#include <unistd.h>\n\
              void\tft_gato_probe(char c) { (void)c; write(1, \"?\", 1); }\n",
        )
        .expect("write");

    let result = harness::run(&PROBE_SUBJECT, &[src_file.path().to_path_buf()])
        .expect("harness::run should not error");

    assert!(!result.is_pass(), "probe should fail with wrong output");
}

//! Subject definitions — one per 42 piscine exercise.
//!
//! Each `Subject` carries static test vectors plus metadata needed to
//! generate the C test harness and run every check.

pub mod c00;

/// A single test case for a C function.
pub struct TestCase {
    /// Human-readable label shown in PASS/FAIL output.
    /// Must describe the *input*, not the expected output.
    pub name: &'static str,
    /// C statement placed verbatim inside a
    /// `static void test_N(void)` wrapper.
    /// May use `printf` — the harness includes `<stdio.h>`.
    pub c_call: &'static str,
    /// SHA-256 commitment of the expected stdout bytes.
    ///
    /// Computed with `sha256sum <<< "expected"` (without trailing
    /// newline) or any standard SHA-256 tool.  The harness captures
    /// the student's stdout, hashes it, and compares — no expected
    /// bytes are ever embedded in generated C code.
    pub expected_sha256: &'static [u8; 32],
}

/// One 42-school piscine exercise.
pub struct Subject {
    /// Folder name, e.g. `"ex00"`.
    pub exercise: &'static str,
    /// Primary function being tested, e.g. `"ft_putchar"`.
    pub function: &'static str,
    /// C prototype line(s) placed at the top of the harness,
    /// e.g. `"void\tft_putchar(char c);"`.
    pub c_prototype: &'static str,
    /// Source files the student must provide (relative to exercise dir).
    pub files: &'static [&'static str],
    /// Functions forbidden in this exercise.
    pub forbidden: &'static [&'static str],
    /// One-line description shown in the TUI and boomer CLI.
    pub description: &'static str,
    /// Test vectors for the harness.
    pub tests: &'static [TestCase],
}

/// All C00 subjects, in order.
pub fn all_c00() -> &'static [Subject] {
    c00::ALL
}

//! Subject definitions — one per 42 piscine exercise.
//!
//! Each `Subject` carries static test vectors plus metadata needed to
//! generate the C test harness and run every check.

pub mod c00;
pub mod c01;
pub mod c02;
pub mod c03;
pub mod c04;
pub mod c05;
pub mod c06;
pub mod c07;
pub mod c08;
pub mod c09;

/// A single test case for a C function.
pub struct TestCase {
    /// Human-readable label shown in PASS/FAIL output.
    ///
    /// Must describe the *input*, not the expected output.
    pub name: &'static str,
    /// C statement placed verbatim inside a
    /// `static void test_N(void)` wrapper.
    /// May use `printf` — the harness includes `<stdio.h>`.
    pub c_call: &'static str,
    /// SHA-256 commitment of the expected stdout bytes.
    ///
    /// Computed with `printf 'expected' | sha256sum` (no trailing
    /// newline unless the function outputs one). The harness captures
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

/// All C01 subjects, in order.
pub fn all_c01() -> &'static [Subject] {
    c01::ALL
}

/// All C02 subjects, in order.
pub fn all_c02() -> &'static [Subject] {
    c02::ALL
}

/// All C03 subjects, in order.
pub fn all_c03() -> &'static [Subject] {
    c03::ALL
}

/// All C04 subjects, in order.
pub fn all_c04() -> &'static [Subject] {
    c04::ALL
}

/// All C05 subjects, in order.
pub fn all_c05() -> &'static [Subject] {
    c05::ALL
}

/// All C06 subjects, in order.
pub fn all_c06() -> &'static [Subject] {
    c06::ALL
}

/// All C07 subjects, in order.
pub fn all_c07() -> &'static [Subject] {
    c07::ALL
}

/// All C08 subjects, in order.
pub fn all_c08() -> &'static [Subject] {
    c08::ALL
}

/// All C09 subjects, in order.
pub fn all_c09() -> &'static [Subject] {
    c09::ALL
}

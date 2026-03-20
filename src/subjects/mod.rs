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
pub mod rush00;

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

/// All Rush00 subjects, in order.
pub fn all_rush() -> &'static [Subject] {
    rush00::ALL
}

/// Resolve a module name to its subject list.
pub fn from_module(module: &str) -> Option<&'static [Subject]> {
    let normalized = module.trim().to_ascii_uppercase();
    match normalized.as_str() {
        "C00" => Some(all_c00()),
        "C01" => Some(all_c01()),
        "C02" => Some(all_c02()),
        "C03" => Some(all_c03()),
        "C04" => Some(all_c04()),
        "C05" => Some(all_c05()),
        "C06" => Some(all_c06()),
        "C07" => Some(all_c07()),
        "C08" => Some(all_c08()),
        "RUSH00" | "RUSH" => Some(all_rush()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::from_module;

    #[test]
    fn resolves_all_piscine_modules() {
        let expected = [
            ("C00", 9usize),
            ("C01", 9),
            ("C02", 13),
            ("C03", 6),
            ("C04", 6),
            ("C05", 9),
            ("C06", 4),
            ("C07", 6),
            ("C08", 6),
        ];

        for (module, count) in expected {
            let subjects = from_module(module).expect("module should resolve");
            assert_eq!(subjects.len(), count, "wrong count for {module}");
        }
    }

    #[test]
    fn resolves_module_case_insensitively() {
        assert!(from_module("c01").is_some());
        assert!(from_module("rush00").is_some());
    }
}

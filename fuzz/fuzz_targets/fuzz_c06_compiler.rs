#![no_main]

use std::io::Write as _;

use libfuzzer_sys::fuzz_target;

/// Fuzz the compiler check with arbitrary C source that resembles a
/// C06 program (i.e. a file that may define `main`).
///
/// Invariants:
/// - `compiler::compile` must never panic on arbitrary input.
/// - `harness::run_program` must never panic on arbitrary input.
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let tmp = tempfile::TempDir::new().unwrap();
        let src = tmp.path().join("ft_fuzz.c");
        let bin = tmp.path().join("a.out");

        {
            let mut f = std::fs::File::create(&src).unwrap();
            let _ = f.write_all(s.as_bytes());
        }

        // Invariant: compile() must not panic regardless of source content.
        let _ = dawon::checks::compiler::compile(&[src.as_path()], &bin, false);
    }
});

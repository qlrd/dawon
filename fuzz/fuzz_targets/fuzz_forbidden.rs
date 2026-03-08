#![no_main]

use std::io::Write as _;

use libfuzzer_sys::fuzz_target;

/// Fuzz the forbidden-function regex scanner with arbitrary C source.
///
/// Invariants:
/// - `forbidden::check()` must never panic.
/// - `ft_` prefixed functions must never appear in violation messages.
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let mut f = tempfile::Builder::new()
            .suffix(".c")
            .tempfile()
            .unwrap();
        let _ = f.write_all(s.as_bytes());

        let result =
            dawon::checks::forbidden::check(f.path(), None, &["printf", "malloc", "free"]);

        // Invariant: a name prefixed with ft_ must never be flagged.
        if let dawon::report::CheckStatus::Fail(msgs) = result.status {
            for msg in &msgs {
                assert!(
                    !msg.contains("ft_printf")
                        && !msg.contains("ft_malloc")
                        && !msg.contains("ft_free"),
                    "ft_-prefixed function was incorrectly flagged: {msg}"
                );
            }
        }
    }
});

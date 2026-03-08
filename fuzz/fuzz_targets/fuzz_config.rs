#![no_main]

use libfuzzer_sys::fuzz_target;

/// Fuzz the .dawon.toml parser with arbitrary UTF-8 input.
///
/// Invariant: `config::load()` must never panic regardless of input.
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let tmp = tempfile::TempDir::new().unwrap();
        let _ = std::fs::write(tmp.path().join(".dawon.toml"), s);
        let _ = dawon::config::load(tmp.path());
    }
});

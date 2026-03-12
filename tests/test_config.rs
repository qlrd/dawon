//! Rust integration tests for .dawon.toml config loading.

use dawon::config;

#[test]
fn missing_file_returns_defaults() {
    let tmp = tempfile::TempDir::new().unwrap();
    let cfg = config::load(tmp.path()).unwrap();
    assert!(cfg.project.module.is_none());
    assert!(!cfg.checks.no_sanitizers);
    assert!(!cfg.checks.no_valgrind);
}

#[test]
fn valid_config_parsed_correctly() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join(".dawon.toml"),
        r#"
[project]
module = "C00"

[checks]
no_valgrind = true
"#,
    )
    .unwrap();

    let cfg = config::load(tmp.path()).unwrap();
    assert_eq!(cfg.project.module.as_deref(), Some("C00"));
    assert!(cfg.checks.no_valgrind);
    assert!(!cfg.checks.no_sanitizers);
}

#[test]
fn malformed_toml_returns_error() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".dawon.toml"), "not toml {{{{").unwrap();
    let err = config::load(tmp.path()).expect_err("malformed toml should error");
    assert!(err.to_string().starts_with("invalid .dawon.toml:"));
}

#[test]
fn empty_toml_file_returns_defaults() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".dawon.toml"), "").unwrap();
    let cfg = config::load(tmp.path()).unwrap();
    assert!(cfg.project.module.is_none());
}

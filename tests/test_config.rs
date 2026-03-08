//! Rust integration tests for .dawon.toml config loading.

use dawon::config;

#[test]
fn missing_file_returns_defaults() {
    let tmp = tempfile::TempDir::new().unwrap();
    let cfg = config::load(tmp.path()).unwrap();
    assert!(cfg.project.module.is_none());
    assert!(!cfg.checks.no_sanitizers);
    assert!(!cfg.checks.no_valgrind);
    assert!(cfg.checks.extra_forbidden.is_empty());
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
extra_forbidden = ["exit", "abort"]
"#,
    )
    .unwrap();

    let cfg = config::load(tmp.path()).unwrap();
    assert_eq!(cfg.project.module.as_deref(), Some("C00"));
    assert!(cfg.checks.no_valgrind);
    assert!(!cfg.checks.no_sanitizers);
    assert_eq!(cfg.checks.extra_forbidden, vec!["exit", "abort"]);
}

#[test]
fn malformed_toml_returns_error() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".dawon.toml"), "not toml {{{{").unwrap();
    assert!(config::load(tmp.path()).is_err());
}

#[test]
fn empty_toml_file_returns_defaults() {
    let tmp = tempfile::TempDir::new().unwrap();
    std::fs::write(tmp.path().join(".dawon.toml"), "").unwrap();
    let cfg = config::load(tmp.path()).unwrap();
    assert!(cfg.project.module.is_none());
}

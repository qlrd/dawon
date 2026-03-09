//! Integration tests for evaluation pipeline composition.

use dawon::config::Config;
use dawon::eval;
use dawon::report::CheckStatus;
use dawon::subjects;

#[test]
fn run_without_symbol_or_valgrind_only_runs_harness() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ex_dir = tmp.path().join("ex00");
    std::fs::create_dir_all(&ex_dir).unwrap();
    std::fs::write(
        ex_dir.join("ft_putchar.c"),
        r#"
#include <unistd.h>
void ft_putchar(char c) { write(1, &c, 1); }
"#,
    )
    .unwrap();

    let subject = &subjects::all_c00()[0];
    let mut cfg = Config::default();
    cfg.checks.no_valgrind = true;
    let result = eval::run(subject, &ex_dir, &cfg);

    assert_eq!(result.checks.len(), 1);
    assert!(result.checks[0].name.starts_with("Function tests"));
    assert!(matches!(result.checks[0].status, CheckStatus::Pass));
}

#[test]
fn run_with_symbol_flag_includes_symbol_check() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ex_dir = tmp.path().join("ex00");
    std::fs::create_dir_all(&ex_dir).unwrap();
    std::fs::write(
        ex_dir.join("ft_putchar.c"),
        r#"
#include <unistd.h>
void ft_putchar(char c) { write(1, &c, 1); }
"#,
    )
    .unwrap();

    let subject = &subjects::all_c00()[0];
    let mut cfg = Config::default();
    cfg.checks.no_valgrind = true;
    cfg.checks.check_symbol = true;
    let result = eval::run(subject, &ex_dir, &cfg);

    assert_eq!(result.checks.len(), 2);
    assert!(result.checks[0].name.contains("Symbol"));
    assert!(result.checks[1].name.starts_with("Function tests"));
    assert!(matches!(result.checks[0].status, CheckStatus::Pass));
    assert!(matches!(result.checks[1].status, CheckStatus::Pass));
}

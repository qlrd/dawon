//! Rust integration tests for the forbidden-function check.

use std::io::Write as _;

use dawon::checks::forbidden;

fn write_c(content: &str) -> tempfile::NamedTempFile {
    let mut f = tempfile::Builder::new()
        .suffix(".c")
        .tempfile()
        .expect("tempfile");
    f.write_all(content.as_bytes()).expect("write");
    f
}

// ── regex layer ────────────────────────────────────────────────────

#[test]
fn empty_forbidden_list_always_passes() {
    let f = write_c("void ft_putchar(char c) { write(1, &c, 1); }\n");
    assert!(forbidden::check(f.path(), None, &[]).is_pass());
}

#[test]
fn printf_call_detected() {
    let f = write_c("void ft_putchar(char c) { printf(\"%c\", c); }\n");
    assert!(!forbidden::check(f.path(), None, &["printf"]).is_pass());
}

#[test]
fn malloc_and_free_both_detected() {
    let f = write_c("void f(void) { char *p = malloc(10); free(p); }\n");
    assert!(!forbidden::check(f.path(), None, &["malloc", "free"]).is_pass());
}

#[test]
fn ft_prefix_not_flagged() {
    // ft_printf is a student function, not the real printf
    let f = write_c("int ft_printf(char *s) { return 0; }\n");
    assert!(forbidden::check(f.path(), None, &["printf"]).is_pass());
}

#[test]
fn call_with_space_before_paren_detected() {
    let f = write_c("void f(void) { printf (\"hi\"); }\n");
    assert!(!forbidden::check(f.path(), None, &["printf"]).is_pass());
}

#[test]
fn commented_out_call_not_flagged() {
    // Single-line comment: the scanner strips // ... before matching
    let f = write_c("void f(void) { // printf(\"hi\");\n}\n");
    assert!(forbidden::check(f.path(), None, &["printf"]).is_pass());
}

#[test]
fn non_c_file_ignored() {
    // check() skips files that don't end in .c
    let f = tempfile::Builder::new()
        .suffix(".h")
        .tempfile()
        .expect("tempfile");
    assert!(forbidden::check(f.path(), None, &["printf"]).is_pass());
}

// ── failure message content ────────────────────────────────────────

#[test]
fn failure_message_contains_function_name() {
    let f = write_c("void g(void) { malloc(1); }\n");
    let result = forbidden::check(f.path(), None, &["malloc"]);
    match result.status {
        dawon::report::CheckStatus::Fail(msgs) => {
            assert!(msgs.iter().any(|m| m.contains("malloc")));
        }
        other => panic!("expected Fail, got {other:?}"),
    }
}

#[test]
fn failure_message_contains_line_number() {
    let f = write_c("void g(void)\n{\n    malloc(1);\n}\n");
    let result = forbidden::check(f.path(), None, &["malloc"]);
    match result.status {
        dawon::report::CheckStatus::Fail(msgs) => {
            // Line 3 has the malloc call
            assert!(msgs.iter().any(|m| m.contains('3')));
        }
        other => panic!("expected Fail, got {other:?}"),
    }
}

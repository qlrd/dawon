//! C06 exercise definitions — argc and argv handling.
//!
//! Functions under test take explicit argc/argv arrays so the
//! harness can supply known inputs without modifying main().
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
        function: "ft_print_program_name",
        c_prototype: "void\tft_print_program_name(int argc, char **argv);",
        files: &["ft_print_program_name.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print argv[0] followed by a newline.",
        tests: &[TestCase {
            name: "argv[0] = \"test\"",
            c_call: "char *av[] = {\"test\", NULL}; ft_print_program_name(1, av);",
            expected_sha256: &hex!(
                "f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2"
            ),
        }],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_print_params",
        c_prototype: "void\tft_print_params(int argc, char **argv);",
        files: &["ft_print_params.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print argv[1] to argv[argc-1], each on its own line.",
        tests: &[
            TestCase {
                name: "\"hello\" \"world\"",
                c_call: "char *av[] = {\"test\", \"hello\", \"world\", NULL}; \
                         ft_print_params(3, av);",
                expected_sha256: &hex!(
                    "4a1e67f2fe1d1cc7b31d0ca2ec441da4778203a036a77da10344c85e24ff0f92"
                ),
            },
            TestCase {
                name: "no params",
                c_call: "char *av[] = {\"test\", NULL}; ft_print_params(1, av);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_rev_params",
        c_prototype: "void\tft_rev_params(int argc, char **argv);",
        files: &["ft_rev_params.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print parameters in reverse order, each on its own line.",
        tests: &[
            TestCase {
                name: "\"hello\" \"world\" reversed",
                c_call: "char *av[] = {\"test\", \"hello\", \"world\", NULL}; \
                         ft_rev_params(3, av);",
                expected_sha256: &hex!(
                    "f38e53b955db4a2f9b7ebfc43036df81be2766718905b855a83431525e480442"
                ),
            },
            TestCase {
                name: "no params",
                c_call: "char *av[] = {\"test\", NULL}; ft_rev_params(1, av);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_sort_params",
        c_prototype: "void\tft_sort_params(int argc, char **argv);",
        files: &["ft_sort_params.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print parameters sorted in ASCII order, each on its own line.",
        tests: &[
            TestCase {
                name: "\"banana\" \"apple\" \"cherry\"",
                c_call: "char *av[] = {\"test\", \"banana\", \"apple\", \"cherry\", NULL}; \
                         ft_sort_params(4, av);",
                expected_sha256: &hex!(
                    "6883ada0490e1bd845b6032e95119d522212f98b840eef466ebb09f4a9eb7a03"
                ),
            },
            TestCase {
                name: "no params",
                c_call: "char *av[] = {\"test\", NULL}; ft_sort_params(1, av);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
];

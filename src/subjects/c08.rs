//! C08 exercise definitions — header files.
//!
//! C08 exercises require students to write `.h` files rather than
//! function implementations. The harness tests what it can:
//! exercises that expose callable C functions or macros.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex02 ───────────────────────────────────────────────────────
    // ex00 (ft_boolean.h) and ex01 (ft_abs.h) define macros only —
    // no linkable .c file to test.
    Subject {
        exercise: "ex02",
        function: "ft_strs_to_tab",
        c_prototype: "char\t**ft_strs_to_tab(unsigned int size, char *strs);",
        files: &["ft_strs_to_tab.c"],
        forbidden: &[],
        description: "Split a string by spaces into a malloc'd array of strings.",
        tests: &[TestCase {
            name: "\"hello world\"",
            c_call: "char **t = ft_strs_to_tab(11, \"hello world\"); \
                         printf(\"%s\\n%s\\n\", t[0], t[1]); \
                         free(t[0]); free(t[1]); free(t);",
            expected_sha256: &hex!(
                "4a1e67f2fe1d1cc7b31d0ca2ec441da4778203a036a77da10344c85e24ff0f92"
            ),
        }],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_show_tab",
        c_prototype: "void\tft_show_tab(char const **tab, unsigned int size);",
        files: &["ft_show_tab.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print each string in tab followed by a newline.",
        tests: &[
            TestCase {
                name: "\"hello\" \"world\"",
                c_call: "char const *t[] = {\"hello\", \"world\"}; \
                         ft_show_tab(t, 2);",
                expected_sha256: &hex!(
                    "4a1e67f2fe1d1cc7b31d0ca2ec441da4778203a036a77da10344c85e24ff0f92"
                ),
            },
            TestCase {
                name: "empty table",
                c_call: "ft_show_tab(NULL, 0);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
];

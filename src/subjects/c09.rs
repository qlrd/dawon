//! C09 exercise definitions — libft (subset).
//!
//! C09 is the libft project: students write a library with many
//! re-implementations of standard C functions plus extra utilities.
//! Only a representative subset is included here; the Makefile
//! exercise (ex00) is tested via compilation only.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex01 ───────────────────────────────────────────────────────
    // ex00 is a Makefile exercise — no harness test possible.
    Subject {
        exercise: "ex01",
        function: "ft_putchar",
        c_prototype: "void\tft_putchar(char c);",
        files: &["ft_putchar.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Write one char to stdout.",
        tests: &[
            TestCase {
                name: "'A'",
                c_call: "ft_putchar('A');",
                expected_sha256: &hex!(
                    "559aead08264d5795d3909718cdd05abd49572e84fe55590eef31a88a08fdffd"
                ),
            },
            TestCase {
                name: "'\\0'",
                c_call: "ft_putchar('\\0');",
                expected_sha256: &hex!(
                    "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d"
                ),
            },
        ],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_putstr",
        c_prototype: "void\tft_putstr(char *str);",
        files: &["ft_putstr.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Write a string to stdout.",
        tests: &[
            TestCase {
                name: "\"hello\"",
                c_call: "ft_putstr(\"hello\");",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "ft_putstr(\"\");",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_strlen",
        c_prototype: "int\tft_strlen(char *str);",
        files: &["ft_strlen.c"],
        forbidden: &["strlen"],
        description: "Return the length of the string.",
        tests: &[
            TestCase {
                name: "empty string",
                c_call: "printf(\"%d\\n\", ft_strlen(\"\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "\"hello\"",
                c_call: "printf(\"%d\\n\", ft_strlen(\"hello\"));",
                expected_sha256: &hex!(
                    "f0b5c2c2211c8d67ed15e75e656c7862d086e9245420892a7de62cd9ec582a06"
                ),
            },
        ],
    },
    // ── ex04 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex04",
        function: "ft_strcpy",
        c_prototype: "char\t*ft_strcpy(char *dest, char *src);",
        files: &["ft_strcpy.c"],
        forbidden: &["strcpy", "strncpy"],
        description: "Copy src into dest including the null terminator.",
        tests: &[
            TestCase {
                name: "\"hello\"",
                c_call: "char buf[50]; ft_strcpy(buf, \"hello\"); printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "char buf[50]; ft_strcpy(buf, \"\"); printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex05 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex05",
        function: "ft_strcmp",
        c_prototype: "int\tft_strcmp(char *s1, char *s2);",
        files: &["ft_strcmp.c"],
        forbidden: &["strcmp", "strncmp"],
        description: "Lexicographic comparison of two strings.",
        tests: &[
            TestCase {
                name: "\"abc\" vs \"abc\"",
                c_call: "printf(\"%d\\n\", ft_strcmp(\"abc\", \"abc\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "\"abc\" vs \"abd\"",
                c_call: "printf(\"%d\\n\", ft_strcmp(\"abc\", \"abd\"));",
                expected_sha256: &hex!(
                    "ee3aa64bb94a50845d5024cd4bd20202a4567aed5cd5328c0d97e9920775fc28"
                ),
            },
        ],
    },
    // ── ex06 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex06",
        function: "ft_putnbr",
        c_prototype: "void\tft_putnbr(int nb);",
        files: &["ft_putnbr.c"],
        forbidden: &[
            "printf", "putchar", "puts", "fprintf", "sprintf", "itoa", "wprintf",
        ],
        description: "Print an integer in base 10. INT_MIN must not overflow.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "ft_putnbr(0);",
                expected_sha256: &hex!(
                    "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
                ),
            },
            TestCase {
                name: "42",
                c_call: "ft_putnbr(42);",
                expected_sha256: &hex!(
                    "73475cb40a568e8da8a045ced110137e159f890ac4da883b6b17dc651b3a8049"
                ),
            },
            TestCase {
                name: "INT_MIN",
                c_call: "ft_putnbr(INT_MIN);",
                expected_sha256: &hex!(
                    "56bb3b3a6aa1747def7c225256374c5e73f2fc46555adc47ea16e2d782159387"
                ),
            },
        ],
    },
    // ── ex07 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex07",
        function: "ft_itoa",
        c_prototype: "char\t*ft_itoa(int nbr);",
        files: &["ft_itoa.c"],
        forbidden: &["itoa", "sprintf", "snprintf"],
        description: "Return malloc'd string representation of nbr.",
        tests: &[
            TestCase {
                name: "42",
                c_call: "char *s = ft_itoa(42); printf(\"%s\", s); free(s);",
                expected_sha256: &hex!(
                    "73475cb40a568e8da8a045ced110137e159f890ac4da883b6b17dc651b3a8049"
                ),
            },
            TestCase {
                name: "INT_MIN",
                c_call: "char *s = ft_itoa(INT_MIN); printf(\"%s\", s); free(s);",
                expected_sha256: &hex!(
                    "56bb3b3a6aa1747def7c225256374c5e73f2fc46555adc47ea16e2d782159387"
                ),
            },
        ],
    },
];

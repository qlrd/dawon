//! C03 exercise definitions — string comparison and concatenation.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
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
            TestCase {
                name: "\"abd\" vs \"abc\"",
                c_call: "printf(\"%d\\n\", ft_strcmp(\"abd\", \"abc\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "empty vs empty",
                c_call: "printf(\"%d\\n\", ft_strcmp(\"\", \"\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_strncmp",
        c_prototype: "int\tft_strncmp(char *s1, char *s2, unsigned int n);",
        files: &["ft_strncmp.c"],
        forbidden: &["strcmp", "strncmp"],
        description: "Compare first n bytes of two strings.",
        tests: &[
            TestCase {
                name: "\"abc\" vs \"abd\", n=2",
                c_call: "printf(\"%d\\n\", ft_strncmp(\"abc\", \"abd\", 2));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "\"abc\" vs \"abd\", n=3",
                c_call: "printf(\"%d\\n\", ft_strncmp(\"abc\", \"abd\", 3));",
                expected_sha256: &hex!(
                    "ee3aa64bb94a50845d5024cd4bd20202a4567aed5cd5328c0d97e9920775fc28"
                ),
            },
            TestCase {
                name: "\"abc\" vs \"abc\", n=0",
                c_call: "printf(\"%d\\n\", ft_strncmp(\"abc\", \"abc\", 0));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_strcat",
        c_prototype: "char\t*ft_strcat(char *dest, char *src);",
        files: &["ft_strcat.c"],
        forbidden: &["strcat", "strncat"],
        description: "Append src to the end of dest.",
        tests: &[
            TestCase {
                name: "\"hello\" + \" world\"",
                c_call: "char buf[50] = \"hello\"; ft_strcat(buf, \" world\"); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
                ),
            },
            TestCase {
                name: "empty + \"world\"",
                c_call: "char buf[50] = \"\"; ft_strcat(buf, \"world\"); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7"
                ),
            },
            TestCase {
                name: "\"hello\" + empty",
                c_call: "char buf[50] = \"hello\"; ft_strcat(buf, \"\"); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_strncat",
        c_prototype: "char\t*ft_strncat(char *dest, char *src, unsigned int nb);",
        files: &["ft_strncat.c"],
        forbidden: &["strcat", "strncat"],
        description: "Append at most nb bytes of src to dest.",
        tests: &[
            TestCase {
                name: "\"hello\" + \" world\", n=6",
                c_call: "char buf[50] = \"hello\"; ft_strncat(buf, \" world\", 6); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
                ),
            },
            TestCase {
                name: "\"hello\" + \" world\", n=3",
                c_call: "char buf[50] = \"hello\"; ft_strncat(buf, \" world\", 3); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "2b657d6cab6688d5fa741fa77303f6a4cef209da8adbe6f046dced1b44c2f892"
                ),
            },
        ],
    },
    // ── ex04 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex04",
        function: "ft_strstr",
        c_prototype: "char\t*ft_strstr(char *str, char *to_find);",
        files: &["ft_strstr.c"],
        forbidden: &["strstr"],
        description: "Find first occurrence of to_find in str.",
        tests: &[
            TestCase {
                name: "\"hello world\" / \"world\"",
                c_call: "char *r = ft_strstr(\"hello world\", \"world\"); \
                         if (r) printf(\"%s\", r);",
                expected_sha256: &hex!(
                    "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7"
                ),
            },
            TestCase {
                name: "\"hello\" / \"xyz\"",
                c_call: "char *r = ft_strstr(\"hello\", \"xyz\"); \
                         if (r) printf(\"%s\", r);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
            TestCase {
                name: "\"hello\" / empty",
                c_call: "char *r = ft_strstr(\"hello\", \"\"); \
                         if (r) printf(\"%s\", r);",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
        ],
    },
];

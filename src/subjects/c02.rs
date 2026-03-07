//! C02 exercise definitions — string predicates and transformations.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
        function: "ft_strncpy",
        c_prototype: "char\t*ft_strncpy(char *dest, char *src, unsigned int n);",
        files: &["ft_strncpy.c"],
        forbidden: &["strcpy", "strncpy"],
        description: "Copy at most n bytes of src into dest.",
        tests: &[
            TestCase {
                name: "\"hello\", n=5",
                c_call: "char buf[50] = {0}; ft_strncpy(buf, \"hello\", 5); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
            TestCase {
                name: "\"hello\", n=3",
                c_call: "char buf[50] = {0}; ft_strncpy(buf, \"hello\", 3); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "d6a81f224bbf2f7c22baddbd5d40730eb20cfb0b3d74e10cab61788214caceb1"
                ),
            },
        ],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_str_is_alpha",
        c_prototype: "int\tft_str_is_alpha(char *str);",
        files: &["ft_str_is_alpha.c"],
        forbidden: &["isalpha"],
        description: "Return 1 if all chars are alphabetic, 0 otherwise.",
        tests: &[
            TestCase {
                name: "\"abc\"",
                c_call: "printf(\"%d\\n\", ft_str_is_alpha(\"abc\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "\"abc1\"",
                c_call: "printf(\"%d\\n\", ft_str_is_alpha(\"abc1\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "\"ABC\"",
                c_call: "printf(\"%d\\n\", ft_str_is_alpha(\"ABC\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "printf(\"%d\\n\", ft_str_is_alpha(\"\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
        ],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_str_is_numeric",
        c_prototype: "int\tft_str_is_numeric(char *str);",
        files: &["ft_str_is_numeric.c"],
        forbidden: &["isdigit"],
        description: "Return 1 if all chars are digits, 0 otherwise.",
        tests: &[
            TestCase {
                name: "\"123\"",
                c_call: "printf(\"%d\\n\", ft_str_is_numeric(\"123\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "\"12a\"",
                c_call: "printf(\"%d\\n\", ft_str_is_numeric(\"12a\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "printf(\"%d\\n\", ft_str_is_numeric(\"\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_str_is_lowercase",
        c_prototype: "int\tft_str_is_lowercase(char *str);",
        files: &["ft_str_is_lowercase.c"],
        forbidden: &["islower"],
        description: "Return 1 if all chars are lowercase letters, 0 otherwise.",
        tests: &[
            TestCase {
                name: "\"abc\"",
                c_call: "printf(\"%d\\n\", ft_str_is_lowercase(\"abc\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "\"Abc\"",
                c_call: "printf(\"%d\\n\", ft_str_is_lowercase(\"Abc\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "\"123\"",
                c_call: "printf(\"%d\\n\", ft_str_is_lowercase(\"123\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex04 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex04",
        function: "ft_str_is_uppercase",
        c_prototype: "int\tft_str_is_uppercase(char *str);",
        files: &["ft_str_is_uppercase.c"],
        forbidden: &["isupper"],
        description: "Return 1 if all chars are uppercase letters, 0 otherwise.",
        tests: &[
            TestCase {
                name: "\"ABC\"",
                c_call: "printf(\"%d\\n\", ft_str_is_uppercase(\"ABC\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "\"Abc\"",
                c_call: "printf(\"%d\\n\", ft_str_is_uppercase(\"Abc\"));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex05 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex05",
        function: "ft_str_is_printable",
        c_prototype: "int\tft_str_is_printable(char *str);",
        files: &["ft_str_is_printable.c"],
        forbidden: &["isprint"],
        description: "Return 1 if all chars are printable, 0 otherwise.",
        tests: &[
            TestCase {
                name: "\"hello\"",
                c_call: "printf(\"%d\\n\", ft_str_is_printable(\"hello\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "printf(\"%d\\n\", ft_str_is_printable(\"\"));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
        ],
    },
    // ── ex06 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex06",
        function: "ft_strupcase",
        c_prototype: "char\t*ft_strupcase(char *str);",
        files: &["ft_strupcase.c"],
        forbidden: &["toupper"],
        description: "Convert all lowercase chars to uppercase in place.",
        tests: &[
            TestCase {
                name: "\"hello\"",
                c_call: "char s[] = \"hello\"; ft_strupcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "3733cd977ff8eb18b987357e22ced99f46097f31ecb239e878ae63760e83e4d5"
                ),
            },
            TestCase {
                name: "\"Hello World\"",
                c_call: "char s[] = \"Hello World\"; ft_strupcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "787ec76dcafd20c1908eb0936a12f91edd105ab5cd7ecc2b1ae2032648345dff"
                ),
            },
            TestCase {
                name: "\"abc123\"",
                c_call: "char s[] = \"abc123\"; ft_strupcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "e0bebd22819993425814866b62701e2919ea26f1370499c1037b53b9d49c2c8a"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "char s[] = \"\"; ft_strupcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex07 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex07",
        function: "ft_strlowcase",
        c_prototype: "char\t*ft_strlowcase(char *str);",
        files: &["ft_strlowcase.c"],
        forbidden: &["tolower"],
        description: "Convert all uppercase chars to lowercase in place.",
        tests: &[
            TestCase {
                name: "\"HELLO\"",
                c_call: "char s[] = \"HELLO\"; ft_strlowcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
            TestCase {
                name: "\"Hello World\"",
                c_call: "char s[] = \"Hello World\"; ft_strlowcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
                ),
            },
            TestCase {
                name: "\"ABC123\"",
                c_call: "char s[] = \"ABC123\"; ft_strlowcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "6ca13d52ca70c883e0f0bb101e425a89e8624de51db2d2392593af6a84118090"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "char s[] = \"\"; ft_strlowcase(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex08 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex08",
        function: "ft_str_capitalize",
        c_prototype: "char\t*ft_str_capitalize(char *str);",
        files: &["ft_str_capitalize.c"],
        forbidden: &["toupper", "tolower"],
        description: "Capitalize first letter of each word, lowercase the rest.",
        tests: &[
            TestCase {
                name: "\"hello world\"",
                c_call: "char s[] = \"hello world\"; \
                         ft_str_capitalize(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                ),
            },
            TestCase {
                name: "\"the 42 network\"",
                c_call: "char s[] = \"the 42 network\"; \
                         ft_str_capitalize(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "090c93084e307bd624349bdbd9da9f570221fa5faa8cdb3c2daf95486b116093"
                ),
            },
            TestCase {
                name: "\"HELLO WORLD\"",
                c_call: "char s[] = \"HELLO WORLD\"; \
                         ft_str_capitalize(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "char s[] = \"\"; ft_str_capitalize(s); printf(\"%s\", s);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
];

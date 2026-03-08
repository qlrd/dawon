//! C04 exercise definitions — number conversion and string utilities.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
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
            TestCase {
                name: "\"Hello, World!\"",
                c_call: "printf(\"%d\\n\", ft_strlen(\"Hello, World!\"));",
                expected_sha256: &hex!(
                    "1a252402972f6057fa53cc172b52b9ffca698e18311facd0f3b06ecaaef79e17"
                ),
            },
        ],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
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
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_putnbr_base",
        c_prototype: "void\tft_putnbr_base(int nbr, char *base);",
        files: &["ft_putnbr_base.c"],
        forbidden: &[
            "printf", "putchar", "puts", "fprintf", "wprintf", "itoa", "sprintf",
        ],
        description: "Print nbr in the given base to stdout.",
        tests: &[
            TestCase {
                name: "42, base 10",
                c_call: "ft_putnbr_base(42, \"0123456789\");",
                expected_sha256: &hex!(
                    "73475cb40a568e8da8a045ced110137e159f890ac4da883b6b17dc651b3a8049"
                ),
            },
            TestCase {
                name: "0, base 10",
                c_call: "ft_putnbr_base(0, \"0123456789\");",
                expected_sha256: &hex!(
                    "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
                ),
            },
            TestCase {
                name: "-42, base 10",
                c_call: "ft_putnbr_base(-42, \"0123456789\");",
                expected_sha256: &hex!(
                    "fec80006df0542549b4cbaafb8987eee00bb49bca396eefe9ac8be5b5928e8f6"
                ),
            },
            TestCase {
                name: "255, base 16",
                c_call: "ft_putnbr_base(255, \"0123456789ABCDEF\");",
                expected_sha256: &hex!(
                    "096aa8510b0dc13190f1f237db8e29ad6ea34e2f779f2088c7826248535fbf6a"
                ),
            },
            TestCase {
                name: "42, base 2",
                c_call: "ft_putnbr_base(42, \"01\");",
                expected_sha256: &hex!(
                    "2a057642222a878bc360f52f8e1f0dfd2af93196f123269397423155a4ec4884"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_itoa",
        c_prototype: "char\t*ft_itoa(int nbr);",
        files: &["ft_itoa.c"],
        forbidden: &["itoa", "sprintf", "snprintf"],
        description: "Return malloc'd string representation of nbr. INT_MIN edge case.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "char *s = ft_itoa(0); printf(\"%s\", s); free(s);",
                expected_sha256: &hex!(
                    "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
                ),
            },
            TestCase {
                name: "42",
                c_call: "char *s = ft_itoa(42); printf(\"%s\", s); free(s);",
                expected_sha256: &hex!(
                    "73475cb40a568e8da8a045ced110137e159f890ac4da883b6b17dc651b3a8049"
                ),
            },
            TestCase {
                name: "-42",
                c_call: "char *s = ft_itoa(-42); printf(\"%s\", s); free(s);",
                expected_sha256: &hex!(
                    "fec80006df0542549b4cbaafb8987eee00bb49bca396eefe9ac8be5b5928e8f6"
                ),
            },
            TestCase {
                name: "INT_MAX",
                c_call: "char *s = ft_itoa(INT_MAX); printf(\"%s\", s); free(s);",
                expected_sha256: &hex!(
                    "972dcafa6fb4c2c88bce752fca4ab18c6bd88599330a4ad9813915b05bfbe76d"
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

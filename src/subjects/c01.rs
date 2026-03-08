//! C01 exercise definitions — pointers and basic strings.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
        function: "ft_ft",
        c_prototype: "int\t*ft_ft(int *nbr);",
        files: &["ft_ft.c"],
        forbidden: &[],
        description: "Set *nbr to 42 and return the pointer.",
        tests: &[TestCase {
            name: "zero → 42",
            c_call: "int n = 0; ft_ft(&n); printf(\"%d\\n\", n);",
            expected_sha256: &hex!(
                "084c799cd551dd1d8d5c5f9a5d593b2e931f5e36122ee5c793c1d08a19839cc0"
            ),
        }],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_ultimate_ft",
        c_prototype: "int\t******ft_ultimate_ft(int ******nbr);",
        files: &["ft_ultimate_ft.c"],
        forbidden: &[],
        description: "Set ****** nbr to 42 through 6 pointer indirections.",
        tests: &[TestCase {
            name: "zero → 42 through 6 pointers",
            c_call: "int n = 0; \
                     int *p1 = &n; int **p2 = &p1; int ***p3 = &p2; \
                     int ****p4 = &p3; int *****p5 = &p4; int ******p6 = &p5; \
                     ft_ultimate_ft(p6); printf(\"%d\\n\", n);",
            expected_sha256: &hex!(
                "084c799cd551dd1d8d5c5f9a5d593b2e931f5e36122ee5c793c1d08a19839cc0"
            ),
        }],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_swap",
        c_prototype: "void\tft_swap(int *a, int *b);",
        files: &["ft_swap.c"],
        forbidden: &[],
        description: "Swap the values pointed to by a and b.",
        tests: &[
            TestCase {
                name: "1 and 2",
                c_call: "int a = 1, b = 2; ft_swap(&a, &b); printf(\"%d %d\\n\", a, b);",
                expected_sha256: &hex!(
                    "dfac347cb71ae693ba8a54687f7112aff24ec67f88a3c66f69a7bf968fa2ac16"
                ),
            },
            TestCase {
                name: "0 and 0",
                c_call: "int a = 0, b = 0; ft_swap(&a, &b); printf(\"%d %d\\n\", a, b);",
                expected_sha256: &hex!(
                    "0ccdb5a77ba5bf7687f2565a8ed97dfb9c1af45503c496fb646312239fab5101"
                ),
            },
            TestCase {
                name: "5 and -3",
                c_call: "int a = 5, b = -3; ft_swap(&a, &b); printf(\"%d %d\\n\", a, b);",
                expected_sha256: &hex!(
                    "6c9e578066b69b1fcb93316ca74b039433af6df79f547fd31adac2e9c0fc9d91"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_div_mod",
        c_prototype: "void\tft_div_mod(int a, int b, int *div, int *mod);",
        files: &["ft_div_mod.c"],
        forbidden: &[],
        description: "Store a/b in *div and a%b in *mod.",
        tests: &[
            TestCase {
                name: "10 / 3",
                c_call: "int d = 0, m = 0; ft_div_mod(10, 3, &d, &m); \
                         printf(\"%d %d\\n\", d, m);",
                expected_sha256: &hex!(
                    "a279f0fe018bb1305d69ee251cd2a29919d20c910b86e3730fc1b857dd3c573a"
                ),
            },
            TestCase {
                name: "100 / 7",
                c_call: "int d = 0, m = 0; ft_div_mod(100, 7, &d, &m); \
                         printf(\"%d %d\\n\", d, m);",
                expected_sha256: &hex!(
                    "1399c90eca823a7a1569beb1e658dbbe485b960d479ac3ccc7fafeb751987072"
                ),
            },
            TestCase {
                name: "42 / 4",
                c_call: "int d = 0, m = 0; ft_div_mod(42, 4, &d, &m); \
                         printf(\"%d %d\\n\", d, m);",
                expected_sha256: &hex!(
                    "34b914ae86f8b512bd443c4378d6974b4b6e2592c83f2b6687779b652df89821"
                ),
            },
        ],
    },
    // ── ex04 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex04",
        function: "ft_ultimate_div_mod",
        c_prototype: "void\tft_ultimate_div_mod(int *a, int *b);",
        files: &["ft_ultimate_div_mod.c"],
        forbidden: &[],
        description: "Store *a / *b in *a and *a % *b in *b.",
        tests: &[
            TestCase {
                name: "10 / 3",
                c_call: "int a = 10, b = 3; ft_ultimate_div_mod(&a, &b); \
                         printf(\"%d %d\\n\", a, b);",
                expected_sha256: &hex!(
                    "a279f0fe018bb1305d69ee251cd2a29919d20c910b86e3730fc1b857dd3c573a"
                ),
            },
            TestCase {
                name: "42 / 4",
                c_call: "int a = 42, b = 4; ft_ultimate_div_mod(&a, &b); \
                         printf(\"%d %d\\n\", a, b);",
                expected_sha256: &hex!(
                    "34b914ae86f8b512bd443c4378d6974b4b6e2592c83f2b6687779b652df89821"
                ),
            },
        ],
    },
    // ── ex05 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex05",
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
                name: "\"Hello, World!\"",
                c_call: "ft_putstr(\"Hello, World!\");",
                expected_sha256: &hex!(
                    "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f"
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
    // ── ex06 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex06",
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
    // ── ex07 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex07",
        function: "ft_strcpy",
        c_prototype: "char\t*ft_strcpy(char *dest, char *src);",
        files: &["ft_strcpy.c"],
        forbidden: &["strcpy", "strncpy"],
        description: "Copy src into dest including the terminating null byte.",
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
            TestCase {
                name: "\"42\"",
                c_call: "char buf[50]; ft_strcpy(buf, \"42\"); printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "73475cb40a568e8da8a045ced110137e159f890ac4da883b6b17dc651b3a8049"
                ),
            },
        ],
    },
    // ── ex08 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex08",
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
            TestCase {
                name: "\"abc\", n=6",
                c_call: "char buf[50] = {0}; ft_strncpy(buf, \"abc\", 6); \
                         printf(\"%s\", buf);",
                expected_sha256: &hex!(
                    "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
                ),
            },
        ],
    },
];

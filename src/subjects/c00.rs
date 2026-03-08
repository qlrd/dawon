//! C00 subject definitions.
//!
//! Each `TestCase.expected_sha256` is a SHA-256 commitment of the
//! expected stdout bytes.  No plaintext expected output appears here.
//! To verify a commitment: `printf 'expected' | sha256sum`.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
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
                name: "'z'",
                c_call: "ft_putchar('z');",
                expected_sha256: &hex!(
                    "594e519ae499312b29433b7dd8a97ff068defcba9755b6d5d00e84c524d67b06"
                ),
            },
            TestCase {
                name: "'\\n'",
                c_call: "ft_putchar('\\n');",
                expected_sha256: &hex!(
                    "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b"
                ),
            },
            TestCase {
                name: "' '",
                c_call: "ft_putchar(' ');",
                expected_sha256: &hex!(
                    "36a9e7f1c95b82ffb99743e0c5c4ce95d83c9a430aac59f84ef3cbfab6145068"
                ),
            },
            TestCase {
                name: "'0'",
                c_call: "ft_putchar('0');",
                expected_sha256: &hex!(
                    "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
                ),
            },
            TestCase {
                name: "'\\0'",
                c_call: "ft_putchar('\\0');",
                expected_sha256: &hex!(
                    "6e340b9cffb37a989ca544e6bb780a2c78901d3fb33738768511a30617afa01d"
                ),
            },
            TestCase {
                name: "127",
                c_call: "ft_putchar(127);",
                expected_sha256: &hex!(
                    "620bfdaa346b088fb49998d92f19a7eaf6bfc2fb0aee015753966da1028cb731"
                ),
            },
        ],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_print_alphabet",
        c_prototype: "void\tft_print_alphabet(void);",
        files: &["ft_print_alphabet.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Display lowercase letters in a single write call.",
        tests: &[TestCase {
            name: "output",
            c_call: "ft_print_alphabet();",
            expected_sha256: &hex!(
                "71c480df93d6ae2f1efad1447c66c9525e316218cf51fc8d9ed832f2daf18b73"
            ),
        }],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_print_reverse_alphabet",
        c_prototype: "void\tft_print_reverse_alphabet(void);",
        files: &["ft_print_reverse_alphabet.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Display lowercase letters in reverse order.",
        tests: &[TestCase {
            name: "output",
            c_call: "ft_print_reverse_alphabet();",
            expected_sha256: &hex!(
                "93ba56cccdc19890b8d31b4f0e9d81f3eff202ed3c130bd98b39fc160b6554ed"
            ),
        }],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_print_numbers",
        c_prototype: "void\tft_print_numbers(void);",
        files: &["ft_print_numbers.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Display digits in a single write call.",
        tests: &[TestCase {
            name: "output",
            c_call: "ft_print_numbers();",
            expected_sha256: &hex!(
                "84d89877f0d4041efb6bf91a16f0248f2fd573e6af05c19f96bedb9f882f7882"
            ),
        }],
    },
    // ── ex04 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex04",
        function: "ft_is_negative",
        c_prototype: "void\tft_is_negative(int n);",
        files: &["ft_is_negative.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print one letter followed by newline based on sign of n.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "ft_is_negative(0);",
                expected_sha256: &hex!(
                    "24d420934ce116d50c3049354652027fb9d2f00b11135500c6217a9ac49d39d5"
                ),
            },
            TestCase {
                name: "-1",
                c_call: "ft_is_negative(-1);",
                expected_sha256: &hex!(
                    "b749bacbabf458de539109abb18a69128c27db492c8511a6a62c6b2fcc5a0b92"
                ),
            },
            TestCase {
                name: "1",
                c_call: "ft_is_negative(1);",
                expected_sha256: &hex!(
                    "24d420934ce116d50c3049354652027fb9d2f00b11135500c6217a9ac49d39d5"
                ),
            },
            TestCase {
                name: "42",
                c_call: "ft_is_negative(42);",
                expected_sha256: &hex!(
                    "24d420934ce116d50c3049354652027fb9d2f00b11135500c6217a9ac49d39d5"
                ),
            },
            TestCase {
                name: "-42",
                c_call: "ft_is_negative(-42);",
                expected_sha256: &hex!(
                    "b749bacbabf458de539109abb18a69128c27db492c8511a6a62c6b2fcc5a0b92"
                ),
            },
            TestCase {
                name: "INT_MIN",
                c_call: "ft_is_negative(INT_MIN);",
                expected_sha256: &hex!(
                    "b749bacbabf458de539109abb18a69128c27db492c8511a6a62c6b2fcc5a0b92"
                ),
            },
            TestCase {
                name: "INT_MAX",
                c_call: "ft_is_negative(INT_MAX);",
                expected_sha256: &hex!(
                    "24d420934ce116d50c3049354652027fb9d2f00b11135500c6217a9ac49d39d5"
                ),
            },
        ],
    },
    // ── ex05 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex05",
        function: "ft_print_comb",
        c_prototype: "void\tft_print_comb(void);",
        files: &["ft_print_comb.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print all combinations of 3 different digits.",
        tests: &[TestCase {
            name: "output",
            c_call: "ft_print_comb();",
            expected_sha256: &hex!(
                "8f1ab65e9778694bb8ccafbb3f19d1cc54ed61855fc3709c2c0e1581abc74770"
            ),
        }],
    },
    // ── ex06 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex06",
        function: "ft_print_comb2",
        c_prototype: "void\tft_print_comb2(void);",
        files: &["ft_print_comb2.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Print all ordered pairs of 2-digit numbers.",
        tests: &[TestCase {
            name: "output",
            c_call: "ft_print_comb2();",
            expected_sha256: &hex!(
                "08537e4e8a7110b23e7668dbb6bf03aaef440757439b514d95c8dbbfc9c16ec8"
            ),
        }],
    },
    // ── ex07 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex07",
        function: "ft_putnbr",
        c_prototype: "void\tft_putnbr(int nb);",
        files: &["ft_putnbr.c"],
        forbidden: &[
            "printf", "putchar", "puts", "fprintf", "sprintf", "itoa", "wprintf",
        ],
        description: "Print an integer in base 10.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "ft_putnbr(0);",
                expected_sha256: &hex!(
                    "5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"
                ),
            },
            TestCase {
                name: "1",
                c_call: "ft_putnbr(1);",
                expected_sha256: &hex!(
                    "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b"
                ),
            },
            TestCase {
                name: "-1",
                c_call: "ft_putnbr(-1);",
                expected_sha256: &hex!(
                    "1bad6b8cf97131fceab8543e81f7757195fbb1d36b376ee994ad1cf17699c464"
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
                name: "-42",
                c_call: "ft_putnbr(-42);",
                expected_sha256: &hex!(
                    "fec80006df0542549b4cbaafb8987eee00bb49bca396eefe9ac8be5b5928e8f6"
                ),
            },
            TestCase {
                name: "100",
                c_call: "ft_putnbr(100);",
                expected_sha256: &hex!(
                    "ad57366865126e55649ecb23ae1d48887544976efea46a48eb5d85a6eeb4d306"
                ),
            },
            TestCase {
                name: "INT_MAX",
                c_call: "ft_putnbr(INT_MAX);",
                expected_sha256: &hex!(
                    "972dcafa6fb4c2c88bce752fca4ab18c6bd88599330a4ad9813915b05bfbe76d"
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
    // ── ex08 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex08",
        function: "ft_print_combn",
        c_prototype: "void\tft_print_combn(int n);",
        files: &["ft_print_combn.c"],
        forbidden: &["printf", "putchar", "puts", "fprintf", "wprintf"],
        description: "Generalise ft_print_comb for n digits.",
        tests: &[
            TestCase {
                name: "n=1",
                c_call: "ft_print_combn(1);",
                expected_sha256: &hex!(
                    "84d89877f0d4041efb6bf91a16f0248f2fd573e6af05c19f96bedb9f882f7882"
                ),
            },
            TestCase {
                name: "n=2",
                c_call: "ft_print_combn(2);",
                expected_sha256: &hex!(
                    "3464fd58bd65cbfe2c2ec7691d3af3efd83af33ca95912a9656c49f02affcb6c"
                ),
            },
            TestCase {
                name: "n=3",
                c_call: "ft_print_combn(3);",
                expected_sha256: &hex!(
                    "8f1ab65e9778694bb8ccafbb3f19d1cc54ed61855fc3709c2c0e1581abc74770"
                ),
            },
        ],
    },
];

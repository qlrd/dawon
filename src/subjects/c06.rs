//! C06 subject definitions — program-based exercises (argc/argv).
//!
//! Each `ProgramTestCase.expected_sha256` is a SHA-256 commitment of
//! the expected stdout bytes.  The program is compiled to `./a.out`
//! and invoked with `argv` in a temp directory so that `argv[0]` is
//! always `"./a.out"`.
//!
//! To verify a commitment: `printf 'expected' | sha256sum`.

use hex_literal::hex;

use super::{ProgramSubject, ProgramTestCase};

pub static ALL: &[ProgramSubject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    ProgramSubject {
        exercise: "ex00",
        program: "ft_print_program_name",
        files: &["ft_print_program_name.c"],
        description: "Display the program name followed by a newline.",
        tests: &[ProgramTestCase {
            name: "no extra args",
            argv: &[],
            // printf './a.out\n' | sha256sum
            expected_sha256: &hex!(
                "761420cd18b7cf45d6a8d2c373ca7b6d22b988d063dcd208a9a14d7b7bc16f85"
            ),
        }],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    ProgramSubject {
        exercise: "ex01",
        program: "ft_print_params",
        files: &["ft_print_params.c"],
        description: "Display all arguments (except argv[0]) in order, one per line.",
        tests: &[
            ProgramTestCase {
                name: "no args",
                argv: &[],
                // printf '' | sha256sum
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
            ProgramTestCase {
                name: "test1 test2 test3",
                argv: &["test1", "test2", "test3"],
                // printf 'test1\ntest2\ntest3\n' | sha256sum
                expected_sha256: &hex!(
                    "bb673e7722bc7453f4de9da7694966f178ac572c73331cec30335e04562aad21"
                ),
            },
            ProgramTestCase {
                name: "42",
                argv: &["42"],
                // printf '42\n' | sha256sum
                expected_sha256: &hex!(
                    "084c799cd551dd1d8d5c5f9a5d593b2e931f5e36122ee5c793c1d08a19839cc0"
                ),
            },
        ],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    ProgramSubject {
        exercise: "ex02",
        program: "ft_rev_params",
        files: &["ft_rev_params.c"],
        description: "Display all arguments (except argv[0]) in reverse order, one per line.",
        tests: &[
            ProgramTestCase {
                name: "no args",
                argv: &[],
                // printf '' | sha256sum
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
            ProgramTestCase {
                name: "test1 test2 test3",
                argv: &["test1", "test2", "test3"],
                // printf 'test3\ntest2\ntest1\n' | sha256sum
                expected_sha256: &hex!(
                    "0887820b086a438e410d4f80d60f69a978b9d4d1a646130e3b9e2a5373771fa9"
                ),
            },
            ProgramTestCase {
                name: "hello",
                argv: &["hello"],
                // printf 'hello\n' | sha256sum
                expected_sha256: &hex!(
                    "5891b5b522d5df086d0ff0b110fbd9d21bb4fc7163af34d08286a2e846f6be03"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    ProgramSubject {
        exercise: "ex03",
        program: "ft_sort_params",
        files: &["ft_sort_params.c"],
        description: "Display all arguments (except argv[0]) sorted in ASCII order, one per line.",
        tests: &[
            ProgramTestCase {
                name: "no args",
                argv: &[],
                // printf '' | sha256sum
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
            ProgramTestCase {
                name: "banana apple cherry",
                argv: &["banana", "apple", "cherry"],
                // printf 'apple\nbanana\ncherry\n' | sha256sum
                expected_sha256: &hex!(
                    "6883ada0490e1bd845b6032e95119d522212f98b840eef466ebb09f4a9eb7a03"
                ),
            },
            ProgramTestCase {
                name: "def abc (reverse input)",
                argv: &["def", "abc"],
                // printf 'abc\ndef\n' | sha256sum
                expected_sha256: &hex!(
                    "924d391c158a46409fdff363063d718ea0bc00b14556f129984942af91233bbe"
                ),
            },
            ProgramTestCase {
                name: "A B C (already sorted)",
                argv: &["A", "B", "C"],
                // printf 'A\nB\nC\n' | sha256sum
                expected_sha256: &hex!(
                    "706204f15ce1834ad298c8e8d270315652bbd6e40cec489f65802db2fdd03167"
                ),
            },
        ],
    },
];

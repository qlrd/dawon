//! C07 exercise definitions — dynamic memory allocation.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
        function: "ft_strdup",
        c_prototype: "char\t*ft_strdup(char *src);",
        files: &["ft_strdup.c"],
        forbidden: &["strdup"],
        description: "Duplicate src with malloc. Return NULL if malloc fails.",
        tests: &[
            TestCase {
                name: "\"hello\"",
                c_call: "char *d = ft_strdup(\"hello\"); printf(\"%s\", d); free(d);",
                expected_sha256: &hex!(
                    "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
                ),
            },
            TestCase {
                name: "\"42\"",
                c_call: "char *d = ft_strdup(\"42\"); printf(\"%s\", d); free(d);",
                expected_sha256: &hex!(
                    "73475cb40a568e8da8a045ced110137e159f890ac4da883b6b17dc651b3a8049"
                ),
            },
            TestCase {
                name: "empty string",
                c_call: "char *d = ft_strdup(\"\"); printf(\"%s\", d); free(d);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_range",
        c_prototype: "int\t*ft_range(int min, int max);",
        files: &["ft_range.c"],
        forbidden: &[],
        description: "Return malloc'd array [min, min+1, …, max-1], or NULL if min >= max.",
        tests: &[
            TestCase {
                name: "1 to 5",
                c_call: "int *r = ft_range(1, 5); \
                         printf(\"%d\\n%d\\n%d\\n%d\\n\", r[0], r[1], r[2], r[3]); \
                         free(r);",
                expected_sha256: &hex!(
                    "16fbd7d1f18d2fedb247d73edc3bc6aa040f5ab99bd3b48c35b79e543d22179b"
                ),
            },
            TestCase {
                name: "-2 to 3",
                c_call: "int *r = ft_range(-2, 3); \
                         printf(\"%d\\n%d\\n%d\\n%d\\n%d\\n\", \
                                r[0], r[1], r[2], r[3], r[4]); \
                         free(r);",
                expected_sha256: &hex!(
                    "e2533aa090751f1006a74d165aa2d4a7ac23dbf73a65eb624c2a81216cc7dff5"
                ),
            },
            TestCase {
                name: "min >= max → NULL",
                c_call: "int *r = ft_range(5, 3); \
                         if (!r) printf(\"\"); else free(r);",
                expected_sha256: &hex!(
                    "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
                ),
            },
        ],
    },
];

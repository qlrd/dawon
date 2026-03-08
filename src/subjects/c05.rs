//! C05 exercise definitions — recursion and mathematics.
//!
//! All expected outputs are stored as SHA-256 commitment hashes.
//! No plaintext answers appear in this file.

use hex_literal::hex;

use super::{Subject, TestCase};

pub static ALL: &[Subject] = &[
    // ── ex00 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex00",
        function: "ft_iterative_factorial",
        c_prototype: "int\tft_iterative_factorial(int nb);",
        files: &["ft_iterative_factorial.c"],
        forbidden: &[],
        description: "Return nb! iteratively. Return 0 for negative input.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "printf(\"%d\\n\", ft_iterative_factorial(0));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "1",
                c_call: "printf(\"%d\\n\", ft_iterative_factorial(1));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "5",
                c_call: "printf(\"%d\\n\", ft_iterative_factorial(5));",
                expected_sha256: &hex!(
                    "97b912eb4a61df5f806ca6239dde3e1a4f51ad20aced1642cbb83dc510a5fa6b"
                ),
            },
            TestCase {
                name: "10",
                c_call: "printf(\"%d\\n\", ft_iterative_factorial(10));",
                expected_sha256: &hex!(
                    "b983c444e57992b7de8b05f37514c746b9c0ac63deb6fcc043b5bf89c2949e81"
                ),
            },
            TestCase {
                name: "-1",
                c_call: "printf(\"%d\\n\", ft_iterative_factorial(-1));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex01 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex01",
        function: "ft_recursive_factorial",
        c_prototype: "int\tft_recursive_factorial(int nb);",
        files: &["ft_recursive_factorial.c"],
        forbidden: &[],
        description: "Return nb! recursively. Return 0 for negative input.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "printf(\"%d\\n\", ft_recursive_factorial(0));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "5",
                c_call: "printf(\"%d\\n\", ft_recursive_factorial(5));",
                expected_sha256: &hex!(
                    "97b912eb4a61df5f806ca6239dde3e1a4f51ad20aced1642cbb83dc510a5fa6b"
                ),
            },
            TestCase {
                name: "10",
                c_call: "printf(\"%d\\n\", ft_recursive_factorial(10));",
                expected_sha256: &hex!(
                    "b983c444e57992b7de8b05f37514c746b9c0ac63deb6fcc043b5bf89c2949e81"
                ),
            },
            TestCase {
                name: "-1",
                c_call: "printf(\"%d\\n\", ft_recursive_factorial(-1));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex02 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex02",
        function: "ft_iterative_power",
        c_prototype: "int\tft_iterative_power(int nb, int power);",
        files: &["ft_iterative_power.c"],
        forbidden: &[],
        description: "Return nb^power iteratively. Return 0 for negative power.",
        tests: &[
            TestCase {
                name: "2^0",
                c_call: "printf(\"%d\\n\", ft_iterative_power(2, 0));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "2^10",
                c_call: "printf(\"%d\\n\", ft_iterative_power(2, 10));",
                expected_sha256: &hex!(
                    "4f71bb761ace37c88826cd8cc1c948e1cf5b5d0cd153dc651a6efdb3dfb9f2b1"
                ),
            },
            TestCase {
                name: "3^3",
                c_call: "printf(\"%d\\n\", ft_iterative_power(3, 3));",
                expected_sha256: &hex!(
                    "932ab0a0e4191d32c0af7b3f565b7b180dbe9869378abc5816f9add54b806e7f"
                ),
            },
            TestCase {
                name: "5^-1",
                c_call: "printf(\"%d\\n\", ft_iterative_power(5, -1));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex03 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex03",
        function: "ft_recursive_power",
        c_prototype: "int\tft_recursive_power(int nb, int power);",
        files: &["ft_recursive_power.c"],
        forbidden: &[],
        description: "Return nb^power recursively. Return 0 for negative power.",
        tests: &[
            TestCase {
                name: "2^10",
                c_call: "printf(\"%d\\n\", ft_recursive_power(2, 10));",
                expected_sha256: &hex!(
                    "4f71bb761ace37c88826cd8cc1c948e1cf5b5d0cd153dc651a6efdb3dfb9f2b1"
                ),
            },
            TestCase {
                name: "3^3",
                c_call: "printf(\"%d\\n\", ft_recursive_power(3, 3));",
                expected_sha256: &hex!(
                    "932ab0a0e4191d32c0af7b3f565b7b180dbe9869378abc5816f9add54b806e7f"
                ),
            },
            TestCase {
                name: "5^-1",
                c_call: "printf(\"%d\\n\", ft_recursive_power(5, -1));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex04 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex04",
        function: "ft_fibonacci",
        c_prototype: "int\tft_fibonacci(int index);",
        files: &["ft_fibonacci.c"],
        forbidden: &[],
        description: "Return the Fibonacci number at index. Return -1 if index < 0.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "printf(\"%d\\n\", ft_fibonacci(0));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "1",
                c_call: "printf(\"%d\\n\", ft_fibonacci(1));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "5",
                c_call: "printf(\"%d\\n\", ft_fibonacci(5));",
                expected_sha256: &hex!(
                    "f0b5c2c2211c8d67ed15e75e656c7862d086e9245420892a7de62cd9ec582a06"
                ),
            },
            TestCase {
                name: "10",
                c_call: "printf(\"%d\\n\", ft_fibonacci(10));",
                expected_sha256: &hex!(
                    "4c82a221b575ce7fe118b2e8cdf0764bf4ef570a3017e80b6d3438af9095f376"
                ),
            },
            TestCase {
                name: "-1",
                c_call: "printf(\"%d\\n\", ft_fibonacci(-1));",
                expected_sha256: &hex!(
                    "ee3aa64bb94a50845d5024cd4bd20202a4567aed5cd5328c0d97e9920775fc28"
                ),
            },
        ],
    },
    // ── ex05 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex05",
        function: "ft_sqrt",
        c_prototype: "int\tft_sqrt(int nb);",
        files: &["ft_sqrt.c"],
        forbidden: &["sqrt"],
        description: "Return integer square root of nb, or 0 if nb is not a perfect square.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "printf(\"%d\\n\", ft_sqrt(0));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "1",
                c_call: "printf(\"%d\\n\", ft_sqrt(1));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "4",
                c_call: "printf(\"%d\\n\", ft_sqrt(4));",
                expected_sha256: &hex!(
                    "53c234e5e8472b6ac51c1ae1cab3fe06fad053beb8ebfd8977b010655bfdd3c3"
                ),
            },
            TestCase {
                name: "9",
                c_call: "printf(\"%d\\n\", ft_sqrt(9));",
                expected_sha256: &hex!(
                    "1121cfccd5913f0a63fec40a6ffd44ea64f9dc135c66634ba001d10bcf4302a2"
                ),
            },
            TestCase {
                name: "2 (not perfect)",
                c_call: "printf(\"%d\\n\", ft_sqrt(2));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "100",
                c_call: "printf(\"%d\\n\", ft_sqrt(100));",
                expected_sha256: &hex!(
                    "917df3320d778ddbaa5c5c7742bc4046bf803c36ed2b050f30844ed206783469"
                ),
            },
        ],
    },
    // ── ex06 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex06",
        function: "ft_is_prime",
        c_prototype: "int\tft_is_prime(int nb);",
        files: &["ft_is_prime.c"],
        forbidden: &[],
        description: "Return 1 if nb is prime, 0 otherwise.",
        tests: &[
            TestCase {
                name: "0",
                c_call: "printf(\"%d\\n\", ft_is_prime(0));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "1",
                c_call: "printf(\"%d\\n\", ft_is_prime(1));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "2",
                c_call: "printf(\"%d\\n\", ft_is_prime(2));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "7",
                c_call: "printf(\"%d\\n\", ft_is_prime(7));",
                expected_sha256: &hex!(
                    "4355a46b19d348dc2f57c046f8ef63d4538ebb936000f3c9ee954a27460dd865"
                ),
            },
            TestCase {
                name: "4",
                c_call: "printf(\"%d\\n\", ft_is_prime(4));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "42",
                c_call: "printf(\"%d\\n\", ft_is_prime(42));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
            TestCase {
                name: "-5",
                c_call: "printf(\"%d\\n\", ft_is_prime(-5));",
                expected_sha256: &hex!(
                    "9a271f2a916b0b6ee6cecb2426f0b3206ef074578be55d9bc94f6f3fe3ab86aa"
                ),
            },
        ],
    },
    // ── ex07 ───────────────────────────────────────────────────────
    Subject {
        exercise: "ex07",
        function: "ft_find_next_prime",
        c_prototype: "int\tft_find_next_prime(int nb);",
        files: &["ft_find_next_prime.c"],
        forbidden: &[],
        description: "Return smallest prime >= nb.",
        tests: &[
            TestCase {
                name: "1",
                c_call: "printf(\"%d\\n\", ft_find_next_prime(1));",
                expected_sha256: &hex!(
                    "53c234e5e8472b6ac51c1ae1cab3fe06fad053beb8ebfd8977b010655bfdd3c3"
                ),
            },
            TestCase {
                name: "2",
                c_call: "printf(\"%d\\n\", ft_find_next_prime(2));",
                expected_sha256: &hex!(
                    "53c234e5e8472b6ac51c1ae1cab3fe06fad053beb8ebfd8977b010655bfdd3c3"
                ),
            },
            TestCase {
                name: "4",
                c_call: "printf(\"%d\\n\", ft_find_next_prime(4));",
                expected_sha256: &hex!(
                    "f0b5c2c2211c8d67ed15e75e656c7862d086e9245420892a7de62cd9ec582a06"
                ),
            },
            TestCase {
                name: "42",
                c_call: "printf(\"%d\\n\", ft_find_next_prime(42));",
                expected_sha256: &hex!(
                    "0e55092af0746630c98d1b2e0d960617c33f8ea7b55739fd18cb7cd5342a28ca"
                ),
            },
            TestCase {
                name: "-5",
                c_call: "printf(\"%d\\n\", ft_find_next_prime(-5));",
                expected_sha256: &hex!(
                    "53c234e5e8472b6ac51c1ae1cab3fe06fad053beb8ebfd8977b010655bfdd3c3"
                ),
            },
        ],
    },
];

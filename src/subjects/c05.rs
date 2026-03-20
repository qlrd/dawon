//! C05 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft_iterative_factorial",
        c_prototype: "int\tft_iterative_factorial(int nb);",
        files: &["ft_iterative_factorial.c"],
        description: "Compute factorial iteratively.",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_recursive_factorial",
        c_prototype: "int\tft_recursive_factorial(int nb);",
        files: &["ft_recursive_factorial.c"],
        description: "Compute factorial recursively.",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_iterative_power",
        c_prototype: "int\tft_iterative_power(int nb, int power);",
        files: &["ft_iterative_power.c"],
        description: "Compute power iteratively.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_recursive_power",
        c_prototype: "int\tft_recursive_power(int nb, int power);",
        files: &["ft_recursive_power.c"],
        description: "Compute power recursively.",
        tests: &[],
    },
    Subject {
        exercise: "ex04",
        function: "ft_fibonacci",
        c_prototype: "int\tft_fibonacci(int index);",
        files: &["ft_fibonacci.c"],
        description: "Return n-th Fibonacci number.",
        tests: &[],
    },
    Subject {
        exercise: "ex05",
        function: "ft_sqrt",
        c_prototype: "int\tft_sqrt(int nb);",
        files: &["ft_sqrt.c"],
        description: "Return integer square root when exact.",
        tests: &[],
    },
    Subject {
        exercise: "ex06",
        function: "ft_is_prime",
        c_prototype: "int\tft_is_prime(int nb);",
        files: &["ft_is_prime.c"],
        description: "Return 1 if number is prime.",
        tests: &[],
    },
    Subject {
        exercise: "ex07",
        function: "ft_find_next_prime",
        c_prototype: "int\tft_find_next_prime(int nb);",
        files: &["ft_find_next_prime.c"],
        description: "Return smallest prime >= nb.",
        tests: &[],
    },
    Subject {
        exercise: "ex08",
        function: "ft_ten_queens_puzzle",
        c_prototype: "int\tft_ten_queens_puzzle(void);",
        files: &["ft_ten_queens_puzzle.c"],
        description: "Print and count ten-queens solutions.",
        tests: &[],
    },
];

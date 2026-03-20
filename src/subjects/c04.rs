//! C04 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft_strlen",
        c_prototype: "int\tft_strlen(char *str);",
        files: &["ft_strlen.c"],
        description: "Return string length.",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_putstr",
        c_prototype: "void\tft_putstr(char *str);",
        files: &["ft_putstr.c"],
        description: "Print a string to stdout.",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_putnbr",
        c_prototype: "void\tft_putnbr(int nb);",
        files: &["ft_putnbr.c"],
        description: "Print a signed integer in base 10.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_atoi",
        c_prototype: "int\tft_atoi(char *str);",
        files: &["ft_atoi.c"],
        description: "Parse a signed integer from string.",
        tests: &[],
    },
    Subject {
        exercise: "ex04",
        function: "ft_putnbr_base",
        c_prototype: "void\tft_putnbr_base(int nbr, char *base);",
        files: &["ft_putnbr_base.c"],
        description: "Print integer in a custom base alphabet.",
        tests: &[],
    },
    Subject {
        exercise: "ex05",
        function: "ft_atoi_base",
        c_prototype: "int\tft_atoi_base(char *str, char *base);",
        files: &["ft_atoi_base.c"],
        description: "Parse integer from custom base alphabet.",
        tests: &[],
    },
];

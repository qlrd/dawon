//! C01 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft_ft",
        c_prototype: "void\tft_ft(int *nbr);",
        files: &["ft_ft.c"],
        description: "Store 42 in the pointed integer.",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_ultimate_ft",
        c_prototype: "void\tft_ultimate_ft(int *********nbr);",
        files: &["ft_ultimate_ft.c"],
        description: "Store 42 through nine pointer levels.",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_swap",
        c_prototype: "void\tft_swap(int *a, int *b);",
        files: &["ft_swap.c"],
        description: "Swap two integers in place.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_div_mod",
        c_prototype: "void\tft_div_mod(int a, int b, int *div, int *mod);",
        files: &["ft_div_mod.c"],
        description: "Write quotient and remainder via pointers.",
        tests: &[],
    },
    Subject {
        exercise: "ex04",
        function: "ft_ultimate_div_mod",
        c_prototype: "void\tft_ultimate_div_mod(int *a, int *b);",
        files: &["ft_ultimate_div_mod.c"],
        description: "Replace *a with quotient and *b with remainder.",
        tests: &[],
    },
    Subject {
        exercise: "ex05",
        function: "ft_putstr",
        c_prototype: "void\tft_putstr(char *str);",
        files: &["ft_putstr.c"],
        description: "Print a string to stdout.",
        tests: &[],
    },
    Subject {
        exercise: "ex06",
        function: "ft_strlen",
        c_prototype: "int\tft_strlen(char *str);",
        files: &["ft_strlen.c"],
        description: "Return string length.",
        tests: &[],
    },
    Subject {
        exercise: "ex07",
        function: "ft_rev_int_tab",
        c_prototype: "void\tft_rev_int_tab(int *tab, int size);",
        files: &["ft_rev_int_tab.c"],
        description: "Reverse an integer array in place.",
        tests: &[],
    },
    Subject {
        exercise: "ex08",
        function: "ft_sort_int_tab",
        c_prototype: "void\tft_sort_int_tab(int *tab, int size);",
        files: &["ft_sort_int_tab.c"],
        description: "Sort an integer array in ascending order.",
        tests: &[],
    },
];

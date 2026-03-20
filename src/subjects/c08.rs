//! C08 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft.h",
        c_prototype: "",
        files: &["ft.h"],
        description: "Basic header with prototypes and macros.",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_boolean.h",
        c_prototype: "",
        files: &["ft_boolean.h"],
        description: "Boolean header definitions.",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_abs.h",
        c_prototype: "",
        files: &["ft_abs.h"],
        description: "ABS macro header.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_point.h",
        c_prototype: "",
        files: &["ft_point.h"],
        description: "Point struct header.",
        tests: &[],
    },
    Subject {
        exercise: "ex04",
        function: "ft_strs_to_tab",
        c_prototype: "struct s_stock_str\t*ft_strs_to_tab(int ac, char **av);",
        files: &["ft_strs_to_tab.c", "ft_stock_str.h"],
        description: "Convert argv to array of stock_str structs.",
        tests: &[],
    },
    Subject {
        exercise: "ex05",
        function: "ft_show_tab",
        c_prototype: "void\tft_show_tab(struct s_stock_str *par);",
        files: &["ft_show_tab.c", "ft_stock_str.h"],
        description: "Display an array of stock_str structs.",
        tests: &[],
    },
];

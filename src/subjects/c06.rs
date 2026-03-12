//! C06 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft_print_program_name",
        c_prototype: "int\tmain(int argc, char **argv);",
        files: &["ft_print_program_name.c"],
        description: "Print program name from argv[0].",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_print_params",
        c_prototype: "int\tmain(int argc, char **argv);",
        files: &["ft_print_params.c"],
        description: "Print command-line arguments in order.",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_rev_params",
        c_prototype: "int\tmain(int argc, char **argv);",
        files: &["ft_rev_params.c"],
        description: "Print command-line arguments in reverse order.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_sort_params",
        c_prototype: "int\tmain(int argc, char **argv);",
        files: &["ft_sort_params.c"],
        description: "Print command-line arguments sorted.",
        tests: &[],
    },
];

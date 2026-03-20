//! C07 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft_strdup",
        c_prototype: "char\t*ft_strdup(char *src);",
        files: &["ft_strdup.c"],
        description: "Duplicate a string with malloc.",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_range",
        c_prototype: "int\t*ft_range(int min, int max);",
        files: &["ft_range.c"],
        description: "Allocate range [min, max).",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_ultimate_range",
        c_prototype: "int\tft_ultimate_range(int **range, int min, int max);",
        files: &["ft_ultimate_range.c"],
        description: "Allocate range and return its length.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_strjoin",
        c_prototype: "char\t*ft_strjoin(int size, char **strs, char *sep);",
        files: &["ft_strjoin.c"],
        description: "Join string array with separator.",
        tests: &[],
    },
    Subject {
        exercise: "ex04",
        function: "ft_convert_base",
        c_prototype: "char\t*ft_convert_base(char *nbr, char *base_from, char *base_to);",
        files: &["ft_convert_base.c", "ft_atoi_base.c", "ft_putnbr_base.c"],
        description: "Convert number string between bases.",
        tests: &[],
    },
    Subject {
        exercise: "ex05",
        function: "ft_split",
        c_prototype: "char\t**ft_split(char *str, char *charset);",
        files: &["ft_split.c"],
        description: "Split string on delimiter charset.",
        tests: &[],
    },
];

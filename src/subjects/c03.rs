//! C03 subject definitions.

use super::Subject;

pub static ALL: &[Subject] = &[
    Subject {
        exercise: "ex00",
        function: "ft_strcmp",
        c_prototype: "int\tft_strcmp(char *s1, char *s2);",
        files: &["ft_strcmp.c"],
        description: "Compare two strings lexicographically.",
        tests: &[],
    },
    Subject {
        exercise: "ex01",
        function: "ft_strncmp",
        c_prototype: "int\tft_strncmp(char *s1, char *s2, unsigned int n);",
        files: &["ft_strncmp.c"],
        description: "Compare at most n chars from two strings.",
        tests: &[],
    },
    Subject {
        exercise: "ex02",
        function: "ft_strcat",
        c_prototype: "char\t*ft_strcat(char *dest, char *src);",
        files: &["ft_strcat.c"],
        description: "Append src to dest.",
        tests: &[],
    },
    Subject {
        exercise: "ex03",
        function: "ft_strncat",
        c_prototype: "char\t*ft_strncat(char *dest, char *src, unsigned int nb);",
        files: &["ft_strncat.c"],
        description: "Append at most nb chars from src to dest.",
        tests: &[],
    },
    Subject {
        exercise: "ex04",
        function: "ft_strstr",
        c_prototype: "char\t*ft_strstr(char *str, char *to_find);",
        files: &["ft_strstr.c"],
        description: "Locate a substring in a string.",
        tests: &[],
    },
    Subject {
        exercise: "ex05",
        function: "ft_strlcat",
        c_prototype: "unsigned int\tft_strlcat(char *dest, char *src, unsigned int size);",
        files: &["ft_strlcat.c"],
        description: "Append with destination-size accounting.",
        tests: &[],
    },
];

//! Rush00 subject definitions.
//!
//! Rush checks currently use the common static checks (norminette,
//! forbidden functions, compiler, valgrind). Harness vectors are not
//! defined yet for this track.

use super::Subject;

pub static ALL: &[Subject] = &[Subject {
    exercise: "rush00",
    function: "main",
    c_prototype: "int\tmain(void);",
    files: &["main.c"],
    forbidden: &[],
    description: "Rush00 program entry point.",
    tests: &[],
}];

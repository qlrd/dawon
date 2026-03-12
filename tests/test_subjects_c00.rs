use dawon::subjects::all_c00;

#[test]
fn c00_contains_ex09_to_ex13() {
    let subjects = all_c00();
    let entries = [
        ("ex09", "ft_display_file"),
        ("ex10", "ft_strdup"),
        ("ex11", "ft_putstr"),
        ("ex12", "ft_strlen"),
        ("ex13", "ft_strcpy"),
    ];

    for (exercise, function) in entries {
        assert!(
            subjects
                .iter()
                .any(|s| s.exercise == exercise && s.function == function),
            "missing {exercise} ({function})"
        );
    }
}

#[test]
fn c00_subject_count_is_now_14() {
    assert_eq!(all_c00().len(), 14);
}

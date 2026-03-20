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
fn c00_contains_at_least_ex00_to_ex13() {
    let subjects = all_c00();
    assert!(subjects.len() >= 14);
    assert_eq!(subjects.first().map(|s| s.exercise), Some("ex00"));
    assert_eq!(subjects.get(13).map(|s| s.exercise), Some("ex13"));
}

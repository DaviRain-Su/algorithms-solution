pub fn length_of_last_word(s: String) -> i32 {
    if let Some(v) = s.trim().split(" ").collect::<Vec<_>>().last() {
        v.len() as i32
    } else {
        0
    }
}

#[test]
fn test_length_of_last_word() {
    assert_eq!(length_of_last_word("Hello World".into()), 5);
    assert_eq!(length_of_last_word("   fly me   to   the moon  ".into()), 4);
    assert_eq!(length_of_last_word("luffy is still joyboy".into()), 6);
    // assert_eq!(length_of_last_word("luffy is still joyboy".into()), 6);
}

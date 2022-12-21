pub fn longest_palindrome(_s: String) -> String {
    todo!()
}

pub fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

#[test]
#[ignore]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("babad".into()), "bab".to_string());
    assert_eq!(longest_palindrome("cbbd".into()), "bb".to_string());
}

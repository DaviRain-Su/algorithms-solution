//! # 9. Palindrome Number
//!
//! Given an integer x, return true if x is a palindrome , and false otherwise.
//!
//! ## Example 1:
//!```no
//! Input: x = 121
//! Output: true
//! Explanation: 121 reads as 121 from left to right and from right to left.
//!```
//! ## Example 2:
//!```no
//! Input: x = -121
//! Output: false
//! Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//!```
//! ## Example 3:
//! ```no
//! Input: x = 10
//! Output: false
//! Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//! ```
//! ## Constraints:
//!
//!```no
//!-231 <= x <= 231 - 1
//!```
pub fn is_palindrome(x: i32) -> bool {
    let right = x
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0);
    x == right
}

#[test]
fn test_is_palindrome() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(-121), false);
    assert_eq!(is_palindrome(10), false);
}

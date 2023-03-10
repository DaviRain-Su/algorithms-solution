// good solution
// https://leetcode.com/problems/sqrtx/solutions/715772/rust-solutions/?q=rust&orderBy=most_relevant
pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }
    let mut left = 0;
    let mut right = x;
    let mut mid = (left + right) / 2;
    loop {
        if mid <= x / mid && (mid + 1) > x / (mid + 1) {
            return mid;
        } else if mid > x / mid {
            right = mid;
        } else if mid < x / mid {
            left = mid;
        }
        mid = (left + right) / 2;
    }
}

#[test]
fn test_my_sqrt() {
    assert_eq!(my_sqrt(4), 2);
    assert_eq!(my_sqrt(8), 2);
    assert_eq!(my_sqrt(2147395600), 46340);
    assert_eq!(my_sqrt(1), 1);
    assert_eq!(my_sqrt(0), 0);
}

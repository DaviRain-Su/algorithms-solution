//! 217. Contains Duplicate
//! 
//! Given an integer array nums, return true if any value appears at least twice in the array,
//!  and return false if every element is distinct.
//!
//! ## Example 1:
//!
//! Input: nums = [1,2,3,1]
//! Output: true
//! 
//! ## Example 2:
//!
//! Input: nums = [1,2,3,4]
//! Output: false
//!
//!  ## Example 3:
//!
//! Input: nums = [1,1,1,3,3,4,3,2,4,2]
//! Output: true
//!
//! ## Constraints:
//!
//! 1 <= nums.length <= 105
//! -109 <= nums[i] <= 109
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut result: Vec<(i32, i32)> = vec![];
    for num in nums {
        if result.contains(&(num, 1)) {
            return true
        } else {
            result.push((num, 1));
        }
    }
    false
}

#[test]
fn test_contains_duplicate() {
    assert_eq!(contains_duplicate(vec![1,2,3,1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1,1,1,3,3,4,3,2,4,2]), true);
    assert_eq!(contains_duplicate(vec![3, 3]), true);
}
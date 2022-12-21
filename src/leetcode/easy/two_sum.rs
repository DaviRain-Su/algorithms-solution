//！ # 1. Two Sum
//!
//! Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! You can return the answer in any order.
//!
//! ## Example 1:
//!
//! ```no
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//! ```
//!
//! ## Example 2:
//!
//! ```no
//! Input: nums = [3,2,4], target = 6
//! Output: [1,2]
//! ```
//!## Example 3:
//!
//! ```no
//! Input: nums = [3,3], target = 6
//! Output: [0,1]
//! ```
//!
//!## Constraints:
//!
//! ```no
//! 2 <= nums.length <= 104
//! -109 <= nums[i] <= 109
//! -109 <= target <= 109
//! ```
//!Only one valid answer exists.
//!
//!
//!Follow-up: Can you come up with an algorithm that is less than O(n2) time complexity?
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = vec![];
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                result = vec![i as i32, j as i32];
                return result;
            }
        }
    }

    result
}

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3,], 6), vec![0, 1]);
}

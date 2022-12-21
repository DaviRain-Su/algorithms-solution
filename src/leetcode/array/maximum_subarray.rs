//! # 53. Maximum Subarray
//! Given an integer array nums, find the subarray
//! which has the largest sum and return its sum.
//!
//! ## Example 1:
//!```no
//! Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//! Output: 6
//! Explanation: [4,-1,2,1] has the largest sum = 6.
//!```
//! ## Example 2:
//!```no
//! Input: nums = [1]
//! Output: 1
//!```
//! ## Example 3:
//!```no
//! Input: nums = [5,4,-1,7,8]
//! Output: 23
//!```
//! ## Constraints:
//!
//!```no
//! 1 <= nums.length <= 105
//! -104 <= nums[i] <= 104
//!```
//!
//! Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
//!
//! 在 Rust 中，你可以使用线性算法来求解最大子数组问题。
//!
//! 该函数接受一个数组 arr，并返回一个元组，其中包含最大子数组的和、起始位置和结束位置。
//!
//! 算法的基本思想是，从左到右扫描数组，并维护一个当前子数组的和。如果当前子数组的和大于最大子数组的和，则更新最大子数组的和，
//! 并记录最大子数组的结束位置。如果当前子数组的和为负数，则将当前子数组的和清零，并记录新的子数组的起始位置。
//!
//! 例如，对于数组 [-2, 1, -3, 4, -1, 2, 1, -5, 4]，该算法的输出将是 (6, 3, 6)，表示最大子数组为 [4, -1, 2, 1]，其和为 6，起始位置为 3，结束位置为 6。
pub fn max_subarray(arr: &[i32]) -> (i32, usize, usize) {
    let mut max_sum = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut current_sum = 0;

    for i in 0..arr.len() {
        current_sum += arr[i];
        if current_sum > max_sum {
            max_sum = current_sum;
            end = i;
        }
        if current_sum < 0 {
            current_sum = 0;
            start = i + 1;
        }
    }

    (max_sum, start, end)
}

#[test]
fn test_max_sub_array() {
    assert_eq!(max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), (6, 3, 6));
}

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut low, mut high) = (0, nums.len());
        
    while low < high {
        let mid = low + (high - low) / 2; //1
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] > target { //2
            high = mid; // 3
        } else {
            low = mid + 1; //4
        }
    }
    low as i32
}

#[test]
fn test_search_insert() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
}
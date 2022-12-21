pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return Vec::new();
    }
    let mut result = vec![];
    let mut idx = 0;
    let mut start = nums[0];

    loop {
        if idx + 1 >= nums.len() {
            if start == nums[idx] {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, nums[idx]));
            }
            break;
        }

        if nums[idx] + 1 != nums[idx + 1] {
            if start == nums[idx] {
                result.push(format!("{}", start));
            } else {
                result.push(format!("{}->{}", start, nums[idx]));
            }
            start = nums[idx + 1];
        }

        idx += 1;
    }

    result
}

#[test]
fn test_summary_ranges() {
    assert_eq!(
        summary_ranges(vec![0, 1, 2, 4, 5, 7]),
        vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()]
    );
    assert_eq!(
        summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
        vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string()
        ]
    );
    assert_eq!(summary_ranges(vec![-1]), vec!["-1"]);
    assert_eq!(summary_ranges(vec![]), Vec::<String>::new());
}

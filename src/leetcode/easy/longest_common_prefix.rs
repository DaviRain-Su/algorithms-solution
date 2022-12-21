pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".into();
    }
    let mut result = String::from("");

    for item in strs[0].chars() {
        result.push(item);
        for str_item in strs.iter() {
            if !str_item.starts_with(&result) {
                result.pop();
                return result;
            }
        }
    }

    result
}

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        longest_common_prefix(vec!["flower".into(), "flow".into(), "flight".into()]),
        "fl".to_string()
    );
    assert_eq!(
        longest_common_prefix(vec!["dog".into(), "racecar".into(), "car".into()]),
        "".to_string()
    );
}

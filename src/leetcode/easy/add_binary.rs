pub fn add_binary(a: String, b: String) -> String {
    let a = a
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, x)| acc + (x * 2_u128.pow(idx as u32)));

    let b = b
        .chars()
        .map(|c| c.to_string().parse::<u128>().unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, x)| acc + (x * 2_u128.pow(idx as u32)));

    format!("{:b}", a + b)
}

#[test]
fn test_add_binary() {
    assert_eq!(add_binary("11".into(), "1".into()), "100".to_string());
    assert_eq!(
        add_binary("1010".into(), "1011".into()),
        "10101".to_string()
    );
    assert_eq!(add_binary("11".into(), "1".into()), "100".to_string());
}

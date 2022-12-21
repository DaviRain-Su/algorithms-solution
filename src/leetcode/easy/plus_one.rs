pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    // 判断1有没有加上
    let mut flag = false;

    for item in digits.into_iter().rev() {
        if item + 1 == 10 && !flag {
            result.push(0);
        } else if item + 1 != 10 && !flag {
            result.push(item + 1);
            flag = true;
        } else {
            result.push(item);
        }
    }

    if !flag {
        result.push(1);
    }

    result.into_iter().rev().collect()
}

#[test]
fn test_plus_one() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    assert_eq!(plus_one(vec![9]), vec![1, 0]);
}

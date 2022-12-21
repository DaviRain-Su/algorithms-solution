use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let mut sum = 0;
    let vector_s = s.chars().map(|c| c.to_owned()).collect::<Vec<_>>();

    let mut i = 0usize;
    loop {
        if i >= vector_s.len() {
            break;
        }

        match (vector_s.get(i), vector_s.get(i + 1)) {
            (Some(&'I'), Some(&'V')) => {
                sum += 4;
                i += 2;
            }
            (Some(&'I'), Some(&'X')) => {
                sum += 9;
                i += 2;
            }
            (Some(&'X'), Some(&'L')) => {
                sum += 40;
                i += 2;
            }
            (Some(&'X'), Some(&'C')) => {
                sum += 90;
                i += 2;
            }
            (Some(&'C'), Some(&'D')) => {
                sum += 400;
                i += 2;
            }
            (Some(&'C'), Some(&'M')) => {
                sum += 900;
                i += 2;
            }
            (Some(key1), Some(_)) => {
                if let Some(value) = map.get(key1) {
                    sum += *value;
                }
                i += 1;
            }
            (Some(key), None) => {
                if let Some(value) = map.get(key) {
                    sum += *value;
                }
                i += 1;
            }
            (_, _) => {
                i += 1;
            }
        }
    }

    sum
}

#[test]
fn test_roman_to_int() {
    assert_eq!(roman_to_int("III".into()), 3);
    assert_eq!(roman_to_int("LVIII".into()), 58);
    assert_eq!(roman_to_int("MCMXCIV".into()), 1994);
}

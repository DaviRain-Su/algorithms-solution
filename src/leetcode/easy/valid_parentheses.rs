pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for item in s.chars() {
        if stack.is_empty() {
            stack.push(item);
        } else {
            match (stack.last(), item) {
                (Some('('), ')') => {
                    stack.pop();
                }
                (Some('['), ']') => {
                    stack.pop();
                }
                (Some('{'), '}') => {
                    stack.pop();
                }
                (_, _) => {
                    stack.push(item);
                }
            }
        }
    }

    if stack.is_empty() {
        true
    } else {
        false
    }
}

#[test]
fn test_is_valid() {
    assert_eq!(is_valid("()".into()), true);
    assert_eq!(is_valid("()[]{}".into()), true);
    assert_eq!(is_valid("(]".into()), false);
    assert_eq!(is_valid("{[]}".into()), true);
}

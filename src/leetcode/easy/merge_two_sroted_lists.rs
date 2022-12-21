///Definition for singly-linked list.

type Link<T> = Option<Box<ListNode<T>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Link<T>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists<T: Ord>(list1: Link<T>, list2: Link<T>) -> Link<T> {
    let mut list1 = list1;
    let mut list2 = list2;
    let mut head = None;
    let mut tail = &mut head;

    loop {
        match (list1, list2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    list1 = l1.next.take();
                    list2 = Some(l2);
                    tail = &mut tail.insert(l1).next;
                } else {
                    list1 = Some(l1);
                    list2 = l2.next.take();
                    tail = &mut tail.insert(l2).next;
                }
            }
            (l1, l2) => break *tail = l1.or(l2),
        }
    }

    head
}

#[allow(unused_macros)]
macro_rules! list {
    () => { None };
    ($head:expr $(, $val:expr)* $(,)?) => {
        Some(Box::new(ListNode {
            val: $head,
            next: list!($($val),*),
        }))
    };
}

#[test]
fn test_merge_list() {
    let list1 = list!(1, 2, 4);
    let list2 = list!(1, 3, 4);
    let result = list!(1, 1, 2, 3, 4, 4);
    assert_eq!(result, merge_two_lists(list1, list2));
    // list1 = [], list2 = [], output = []
    assert_eq!(list!(), merge_two_lists::<i32>(list!(), list!()));
    // list1 = [], list2 = [0], output = [0]
    assert_eq!(list!(0), merge_two_lists(list!(), list!(0)));
}

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1.is_none() {
            list2
        } else if list2.is_none() {
            list1
        } else if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
            Some(Box::new(
                ListNode {
                    val: list1.as_ref().unwrap().val,
                    next: Solution::merge_two_lists(list1.unwrap().next, list2),
                }
            ))
        } else {
            Some(Box::new(
                ListNode {
                    val: list2.as_ref().unwrap().val,
                    next: Solution::merge_two_lists(list1, list2.unwrap().next),
                }
            ))
        }
    }
}
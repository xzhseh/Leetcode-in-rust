pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::helper_func(l1, l2, 0)
    }

    pub fn helper_func(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carried: i32) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carried == 0 {
            None
        } else {
            Some(Box::new(
                ListNode { 
                    next: Solution::helper_func(
                        l1.and_then(|x| {
                            carried += x.val; 
                            x.next
                        }),
                        l2.and_then(|x| {
                            carried += x.val; 
                            x.next
                        }),
                        carried / 10
                    ),
                    val: carried % 10,
                 }
            ))
        }
    }
}
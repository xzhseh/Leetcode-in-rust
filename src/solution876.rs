pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Solution {
    #[allow(dead_code)]
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ptr_slow = head.clone();
        let mut ptr_fast = head.clone();
        loop {
            if ptr_fast.is_none() || ptr_fast.as_ref().and_then(|x| x.next.as_ref()).is_none() {
                break;
            } else {
                ptr_slow = ptr_slow.and_then(|x| x.next);
                ptr_fast = ptr_fast.and_then(|x| x.next.and_then(|x| x.next));
            }
        }

        ptr_slow
    }
}
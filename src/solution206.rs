pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Use a very clean syntax to solve this problem, which is 'while let'
        let mut pre = None;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        
        pre
    }
}


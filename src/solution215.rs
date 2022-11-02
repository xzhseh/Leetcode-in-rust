pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::with_capacity(k as usize);

        for n in nums {
            if heap.len() == heap.capacity() {
                if *heap.peek().unwrap() > Reverse(n) {
                    heap.pop();
                } else {
                    continue;
                }
            }
            heap.push(Reverse(n));
        }
        heap.peek().unwrap().0
    }
}
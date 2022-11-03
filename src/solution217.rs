pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut current_set: HashSet<i32> = HashSet::new();
        for elem in nums {
            if current_set.contains(&elem) {
                return true;
            } else {
                current_set.insert(elem);
                continue;
            }
        }

        false
    }
}
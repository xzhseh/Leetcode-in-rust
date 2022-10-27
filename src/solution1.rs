pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;   
        let mut return_vec: Vec::<i32> = Vec::new();
        let mut current_hashmap: HashMap::<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if current_hashmap.contains_key(&(target - nums[i])) {
                return_vec.push(current_hashmap[&(target - nums[i])] as i32);
                return_vec.push(i as i32);
            } else {
                current_hashmap.insert(nums[i], i as i32);
            }
        }
        return_vec
    }
}
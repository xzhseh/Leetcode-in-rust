pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut current_hashset: HashMap<i32, i32> = HashMap::new();
        let mut return_vec: Vec<i32> = Vec::new();
        let _tmp = nums1.iter().enumerate().map(|(i, c)| {
            if current_hashset.contains_key(&nums1[i]) {
                current_hashset.entry(nums1[i]).and_modify(|x| *x += 1);
                *c
            } else {
                current_hashset.insert(nums1[i], 1);
                *c
            }
        }).collect::<Vec<i32>>();

        let _tmp = nums2.iter().enumerate().map(|(i, c)| {
            if current_hashset.contains_key(&nums2[i]) && current_hashset[&nums2[i]] != 0 {
                return_vec.push(nums2[i] as i32); // Push into the return_vec
                current_hashset.entry(nums2[i]).and_modify(|x| *x -= 1);
            }
            *c
        }).collect::<Vec<i32>>();

        return_vec
    }
}
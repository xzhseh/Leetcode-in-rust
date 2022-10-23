pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum = nums.iter().rfold(0, |acc, &x| acc + x);
        let mut current_sum = 0;
        for i in 0..nums.len() {
            if nums[i] + current_sum * 2 == total_sum {
                return i as i32;
            } else {
                current_sum += nums[i];
            }
        }
        
        -1
    }
}
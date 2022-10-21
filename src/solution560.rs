pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut return_val = 0;
        
        for i in 0..nums.len() {
            let mut current_sum = 0;
            for j in i..nums.len() {
                current_sum += nums[j];
                if current_sum == k {
                    return_val += 1;
                }
            }
        }

        return_val
    }
}
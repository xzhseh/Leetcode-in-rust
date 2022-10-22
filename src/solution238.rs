pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut return_vec: Vec<i32> = Vec::new();
        let mut left_vec: Vec<i32> = vec![1; nums.len()];
        let mut right_vec: Vec<i32> = vec![1; nums.len()];

        for i in 1..nums.len() {
            left_vec[i] = left_vec[i - 1] * nums[i - 1];
            right_vec[nums.len() - i - 1] = right_vec[nums.len() - i] * nums[nums.len() - i];
        }

        for i in 0..nums.len() {
            return_vec.push(left_vec[i] * right_vec[i]);
        }

        return_vec
    }
}
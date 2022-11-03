pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut max: i32 = nums[0];
        let mut previous_val: i32 = nums[0];
        let mut current_val: i32;
        for (i, elem) in nums.iter().enumerate() {
            if i == 0 { continue; }
            if previous_val > 0 {
                current_val = elem + previous_val;
                previous_val = current_val;
            } else {
                current_val = *elem;
                previous_val = current_val;
            }
            if current_val > max {
                max = current_val;
            }
        }

        max
    }
}
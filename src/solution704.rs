pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        let mut flag = false;

        while left < right {
            let mid = left + (right - left) / 2;
            if target == nums[mid] {
                flag = true;
                left = mid;
                break;
            } else if target > nums[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        if flag {
            left as i32
        } else {
            -1
        }
    }
}
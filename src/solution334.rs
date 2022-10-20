pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = nums[0];
        let mut second = 2147483647;

        for elem in nums {
            if elem > second {
                return true;
            } else if elem > first {
                second = elem;
                continue;
            } else if elem < first {
                first = elem;
                continue;
            }
        }

        false
    }
}
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
	pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
		let mut return_list: Vec<i32> = Vec::new();
		for (i, elem) in nums.iter().enumerate() {
			if i == 0 {
				return_list.push(*elem);
				continue;
			} else {
				return_list.push(*elem + return_list[i - 1]);
			}
		}

		return_list
	}
}
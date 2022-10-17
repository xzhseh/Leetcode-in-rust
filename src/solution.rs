pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
		let mut ptr_one = 0;
		let mut ptr_zero = 0;

		for i in 0..nums.len() {
			if nums[i] == 1 {
				let tmp = nums[i];
				nums[i] = nums[ptr_one];
				nums[ptr_one] = tmp;
				ptr_one += 1;
			} else if nums[i] == 0 {
				let tmp = nums[i];
				nums[i] = nums[ptr_zero];
				nums[ptr_zero] = tmp;
				if ptr_zero < ptr_one {
					let tmp = nums[i];
					nums[i] = nums[ptr_one];
					nums[ptr_one] = tmp;
				}
				ptr_zero += 1;
				ptr_one += 1;
			}
		}
    }
}


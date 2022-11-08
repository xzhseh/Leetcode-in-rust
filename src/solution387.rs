pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
		use std::collections::HashMap;
		let mut return_index: i32 = -1;
        let mut flag = false;
		let mut current_hashset: HashMap<char, i32> = HashMap::new();
		let _tmp = s.chars().map(|x| {
			current_hashset.entry(x).and_modify(|x| *x += 1).or_insert(1);
			x
		}).collect::<Vec<char>>();
		let _tmp = s.chars().enumerate().map(|(i, c)| {
			if current_hashset[&c] == 1 && !flag {
				return_index = i as i32;
                flag = true;
			}
			c
		}).collect::<Vec<char>>();
		return_index as i32
    }
}
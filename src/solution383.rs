pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
		use std::collections::HashMap;
		let mut return_bool = true;
		let mut current_hashset: HashMap<char, i32> = HashMap::new();
		let _tmp = magazine.chars().map(|x| {
			current_hashset.entry(x).and_modify(|x| *x += 1).or_insert(1);
			x
		}).collect::<Vec<char>>();
		let _tmp = ransom_note.chars().map(|x| {
			if current_hashset.contains_key(&x) && current_hashset[&x] != 0 {
				current_hashset.entry(x).and_modify(|x| *x -= 1);
			} else {
				return_bool = false;
			}
			x
		}).collect::<Vec<char>>();
		return_bool
    }
}
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
		let mut return_bool = true;
		let mut current_hashset: HashMap<char, i32> = HashMap::new();
		let _tmp = s.chars().map(|x| {
			current_hashset.entry(x).and_modify(|x| *x += 1).or_insert(1);
			x
		}).collect::<Vec<char>>();
		let _tmp = t.chars().map(|x| {
			if current_hashset.contains_key(&x) && current_hashset[&x] != 0 {
				current_hashset.entry(x).and_modify(|x| *x -= 1);
			} else {
				return_bool = false;
			}
			x
		}).collect::<Vec<char>>();
        for val in current_hashset.values() {
            if *val != 0 {
                return_bool = false;
            } else {
                continue;
            }
        }
		return_bool
    }
}
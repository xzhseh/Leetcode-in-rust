pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_isomorphic(s: String, t: String) -> bool {
        use std::collections::HashMap;
		let mut s_hashmap: HashMap<char, char> = HashMap::new();
		let mut t_hashmap: HashMap<char, char> = HashMap::new();
		for i in 0..s.len() {
			let s_val = s_hashmap.entry(s.as_bytes()[i] as char).or_insert(t.as_bytes()[i] as char);
			let t_val = t_hashmap.entry(t.as_bytes()[i] as char).or_insert(s.as_bytes()[i] as char);
			if *s_val != t.as_bytes()[i] as char || *t_val != s.as_bytes()[i] as char {
				return false;
			}
 		}
		true
    }
}
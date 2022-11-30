pub struct Solution;

/* time: 100.00% space: 70.00% */
impl Solution {
    #[allow(dead_code)]
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashMap;
        let mut current_hashmap: HashMap<&str, char> = HashMap::new();
        let mut second_hashmap: HashMap<char, &str> = HashMap::new();
        let input_vec = s.split(" ").collect::<Vec<&str>>();
        if input_vec.len() != pattern.chars().count() {
            return false;
        }
        let mut count = 0;
        for c in pattern.chars() {
            if current_hashmap.contains_key(&input_vec[count]) && second_hashmap.contains_key(&c) {
                if c != current_hashmap[&input_vec[count]] || input_vec[count] != second_hashmap[&c] {
                    return false;
                } else {
                    count += 1;
                }
            } else if current_hashmap.contains_key(&input_vec[count]) || second_hashmap.contains_key(&c) {
                return false;
            } else {
                current_hashmap.insert(input_vec[count], c);
                second_hashmap.insert(c, input_vec[count]);
                count += 1;
            }
        }

        true
    }
}
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn partition_labels(s: String) -> Vec<i32> {
        use std::collections::HashMap;
        let mut current_hashmap: HashMap<char, usize> = HashMap::new();
        let mut return_vec: Vec<i32> = Vec::new();

        for i in 0..s.len() {
            current_hashmap.entry(s.chars().nth(i).unwrap()).and_modify(|x| *x = i).or_insert(i);
        }

        let mut end: usize = 0;
        let mut current_len: i32 = 1;

        for i in 0..s.len() {
            end = std::cmp::max(end, current_hashmap[&s.chars().nth(i).unwrap()]);
            if i == end {
                return_vec.push(current_len);
                end += 1;
                current_len = 1;
            } else {
                current_len += 1;
            }
        }

        return_vec
    }
}
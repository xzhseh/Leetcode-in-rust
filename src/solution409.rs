pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut current_hashmap: HashMap<char, i32> = HashMap::new();
        let mut max_length = 0;
        let mut flag = false;

        for c in s.chars() {
            current_hashmap.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        for (_key, val) in current_hashmap {
            if val % 2 != 0 {
                flag = true;
            }
            max_length += (val / 2) * 2;
        }

        if flag {
            max_length + 1
        } else {
            max_length
        }
    }
}
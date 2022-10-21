pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i = 0;
        let mut j = 0;

        while i < s.len() && j < t.len() {
            if s.chars().nth(i).unwrap() == t.chars().nth(j).unwrap() {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }

        if i == s.len() {
            true
        } else {
            false
        }
    }
}
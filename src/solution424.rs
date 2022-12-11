pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut cur_vec: Vec<i32> = vec![0; 26]; // Use as the frequency calculator for every letter appear in the string
        let mut max_frequency: i32 = 0;
        let mut right: i32 = 0;
        let mut left: i32 = 0;
        let s_bytes = s.as_bytes();

        while right < s.len() as i32 {
            cur_vec[s_bytes[right as usize] as usize - 'A' as usize] += 1;
            max_frequency = std::cmp::max(max_frequency, cur_vec[s_bytes[right as usize] as usize - 'A' as usize]);
            if right - left + 1 - max_frequency > k {
                cur_vec[s_bytes[left as usize] as usize - 'A' as usize] -= 1;
                left += 1;
            }
            right += 1;
        }

        right - left
    }
}
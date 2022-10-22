pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let mut dp: Vec<Vec<bool>> = vec![vec![false; s.len()]; s.len()];
        let mut begin: usize = 0;
        let mut max_len: usize = 1;
        let tmp_array: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            dp[i][i] = true;
        }

        for len in 2..=s.len() { // j - i + 1 = len
            for i in 0..s.len() { 
                let j = len + i - 1;

                if j >= s.len() || i > j {
                    break;
                }

                if tmp_array[i] != tmp_array[j] {
                    dp[i][j] = false;
                } else {
                    if len <= 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }

                if dp[i][j] && len > max_len {
                    max_len = len;
                    begin = i;
                }
            }
        }

        s[begin..(begin + max_len)].to_string()
    }
}
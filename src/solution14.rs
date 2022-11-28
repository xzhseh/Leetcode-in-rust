pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut return_string = String::new();
        let mut length = strs[0].len();

        for i in 1..strs.len() {
            if strs[i].len() < length {
                length = strs[i].len();
            }
        }

        for i in 0..length {
            let mut flag = true;
            let cur_word = strs[0].chars().nth(i).unwrap();

            for j in 1..strs.len() {
                if strs[j].chars().nth(i).unwrap() != cur_word {
                    flag = false;
                    break;
                }
            }
            
            if !flag {
                break;
            } else {
                return_string.push(cur_word);
            }
        }

        return_string
    }
}
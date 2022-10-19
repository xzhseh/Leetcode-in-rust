pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn add_strings(num1: String, num2: String) -> String {
        let min_len = std::cmp::min(num1.len(), num2.len());
        let mut current_val = 0;
        let mut return_val = String::new();
        for i in 0..min_len {
            let tmp1 = num1[i..(i + 1)].parse::<i32>().unwrap();
            let tmp2 = num2[i..(i + 1)].parse::<i32>().unwrap();
            return_val.push_str(&((tmp1 + tmp2) % 10).to_string());
            current_val = (tmp1 + tmp2 + current_val) / 10;
        }

        return_val
    }
}
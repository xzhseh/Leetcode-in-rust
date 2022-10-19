pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn add_strings(num1: String, num2: String) -> String {
        let min_len = std::cmp::min(num1.len(), num2.len());
        let mut current_val = 0;
        let mut return_val = String::new();

        for i in 0..min_len {
            let tmp1 = num1[(num1.len() - i - 1)..(num1.len() - i)].parse::<i32>().unwrap();
            let tmp2 = num2[(num2.len() - i - 1)..(num2.len() - i)].parse::<i32>().unwrap();
            return_val.push_str(&((tmp1 + tmp2 + current_val) % 10).to_string());
            current_val = (tmp1 + tmp2 + current_val) / 10;
        }

        if num1.len() < num2.len() {
            for i in min_len..num2.len() {
                let tmp2 = num2[(num2.len() - i - 1)..(num2.len() - i)].parse::<i32>().unwrap();
                return_val.push_str(&((tmp2 + current_val) % 10).to_string());
                current_val = (tmp2 + current_val) / 10;
            }
        } else {
            for i in min_len..num1.len() {
                let tmp1 = num1[(num1.len() - i - 1)..(num1.len() - i)].parse::<i32>().unwrap();
                return_val.push_str(&((tmp1 + current_val) % 10).to_string());
                current_val = (tmp1 + current_val) / 10;
            }
        }

        if current_val >= 1 {
                return_val.push_str(&current_val.to_string());
            }

        return_val.chars().rev().collect::<String>()
    }
}
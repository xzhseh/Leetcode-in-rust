pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0".to_string() || num2 == "0".to_string() {
            return String::from("0");
        }
        let mut return_string = String::from("0");
        let num2_array = num2.chars().collect::<Vec<char>>();

        for i in 0..num2_array.len() {
            let mut current_up: u32 = 0;
            let mut current_string = String::new();
            let current_val = num2_array[num2_array.len() - i - 1].to_digit(10).unwrap();

            for j in 0..num1.len() {
                if j == num1.len() - 1 {
                    let num1_val = num1.chars().nth(num1.len() - j - 1).unwrap().to_digit(10).unwrap();
                    let current_num = (current_val * num1_val + current_up) % 10;
                    current_up = (current_val * num1_val + current_up) / 10;
                    current_string.push_str(&current_num.to_string());

                    if current_up != 0 {
                        current_string.push_str(&current_up.to_string());
                    }

                    break;
                }

                let num1_val = num1.chars().nth(num1.len() - j - 1).unwrap().to_digit(10).unwrap();
                let current_num = (current_val * num1_val + current_up) % 10;
                current_up = (current_val * num1_val + current_up) / 10;
                current_string.push_str(&current_num.to_string());
            }

            current_string = current_string.chars().rev().collect::<String>();

            if i != 0 {
                let current_zero = (0..i).map(|_x| "0").collect::<String>();
                current_string.push_str(&current_zero);
            }

            return_string = Solution::add_strings(return_string, current_string);
        }

        return_string
    }     
    // The helper function from solution415.rs
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
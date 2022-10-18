use std::vec;

pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut return_vec: Vec<i32> = vec![1; (row_index + 1) as usize];
        let mut tmp_vec = Vec::new();
        for i in 0..(row_index + 1) {
            if i == 0 || i == 1 {
                continue;
            }
            tmp_vec = return_vec.clone();
            for j in 1..i {
                return_vec[j as usize] = tmp_vec[(j - 1) as usize] + tmp_vec[j as usize];
            }
        }

        return_vec
    }
}
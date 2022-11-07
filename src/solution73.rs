pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
		let row = matrix.len();
		let col = matrix[0].len();
		let mut row_hash = vec![false; row];
		let mut col_hash = vec![false; col];
		for i in 0..row {
			for j in 0..col {
				if matrix[i][j] == 0 {
					row_hash[i] = true;
					col_hash[j] = true;
				} else {
					continue;
				}
			}
		}
		for i in 0..row {
			for j in 0..col {
				if row_hash[i] { 
					matrix[i][j] = 0; 
				} else if col_hash[j] {
					matrix[i][j] = 0;
				} else {
					continue;
				}
			}
		}
    }
}
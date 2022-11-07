pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
		let mut col_hash = vec![vec![false; 9]; 9];
		let mut row_hash = vec![vec![false; 9]; 9];
		let mut box_hash = vec![vec![false; 9]; 9];
		for i in 0..9 {
			for j in 0..9 {
				if board[i][j] == '.' { continue; }
				let tmp_val = board[i][j].to_digit(10).unwrap();
				if col_hash[j][tmp_val as usize - 1] { return false; }
				if row_hash[i][tmp_val as usize - 1] { return false; }
				if box_hash[j / 3 + (i / 3) * 3][tmp_val as usize - 1] { return false; }

				col_hash[j][tmp_val as usize - 1] = true;
				row_hash[i][tmp_val as usize - 1] = true;
				box_hash[j / 3 + (i / 3) * 3][tmp_val as usize - 1] = true;
			}
		}
		true
    }
}